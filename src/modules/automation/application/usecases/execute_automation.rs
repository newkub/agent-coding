use crate::modules::automation::domain::models::issue_pr::{
    AutomationConfig, AutomationWorkflow, StepStatus, WorkflowStatus,
};
use crate::modules::automation::domain::operations::automation_operations::{
    determine_target_branch, extract_labels, generate_branch_name, generate_commit_message,
    generate_pr_body, generate_pr_title,
};
use crate::modules::automation::domain::validators::automation_validators;
use crate::modules::automation::ports::{
    AutomationWorkflowExecutor, AutomationWorkflowRepository, GitHubClient, GitOperations,
};
use crate::shared::kernel::result::AppError;

/// Use case for executing issue-to-PR automation.
pub(crate) struct ExecuteAutomationUseCase<G, H, W>
where
    G: GitOperations,
    H: GitHubClient,
    W: AutomationWorkflowRepository,
{
    git: G,
    github: H,
    workflows: W,
}

impl<G, H, W> ExecuteAutomationUseCase<G, H, W>
where
    G: GitOperations,
    H: GitHubClient,
    W: AutomationWorkflowRepository,
{
    pub(crate) const fn new(git: G, github: H, workflows: W) -> Self {
        Self {
            git,
            github,
            workflows,
        }
    }

    /// Execute the automation workflow and persist its observable state.
    pub(crate) async fn execute(
        &self,
        workflow: &mut AutomationWorkflow,
        config: &AutomationConfig,
    ) -> Result<(), AppError> {
        automation_validators::validate_issue_for_automation(&workflow.issue)?;
        automation_validators::validate_automation_config(config)?;
        automation_validators::validate_repository_access(&workflow.issue.repository)?;

        self.workflows.save(workflow).await?;
        workflow.status = WorkflowStatus::InProgress;
        self.workflows.update(workflow).await?;

        match self.execute_steps(workflow, config).await {
            Ok(()) => {
                workflow.status = WorkflowStatus::Completed;
                self.workflows.update(workflow).await?;
                Ok(())
            }
            Err(error) => {
                workflow.status = WorkflowStatus::Failed;
                if let Some(step) = workflow
                    .steps
                    .iter_mut()
                    .find(|step| matches!(step.status, StepStatus::Running | StepStatus::Pending))
                {
                    step.status = StepStatus::Failed;
                    step.completed_at = Some(chrono::Utc::now());
                    step.error = Some(error.to_string());
                }
                self.workflows.update(workflow).await?;
                Err(error)
            }
        }
    }

    async fn execute_steps(
        &self,
        workflow: &mut AutomationWorkflow,
        config: &AutomationConfig,
    ) -> Result<(), AppError> {
        let step_create_branch = uuid::Uuid::new_v4().to_string();
        let step_commit = uuid::Uuid::new_v4().to_string();
        let step_push = uuid::Uuid::new_v4().to_string();
        let step_create_pr = uuid::Uuid::new_v4().to_string();

        workflow.add_step("Create branch".to_string());
        workflow.add_step("Commit changes".to_string());
        workflow.add_step("Push to remote".to_string());
        workflow.add_step("Create pull request".to_string());

        workflow.steps[0].id = step_create_branch.clone();
        workflow.steps[1].id = step_commit.clone();
        workflow.steps[2].id = step_push.clone();
        workflow.steps[3].id = step_create_pr.clone();
        self.workflows.update(workflow).await?;

        workflow.update_step(&step_create_branch, StepStatus::Running, None);
        self.workflows.update(workflow).await?;
        let branch_name = generate_branch_name(&workflow.issue, config);

        if config.auto_create_branch {
            if self.git.branch_exists(&branch_name).await? {
                self.git.checkout_branch(&branch_name).await?;
            } else {
                self.git.create_branch(&branch_name).await?;
                self.git.checkout_branch(&branch_name).await?;
            }
            workflow.update_step(&step_create_branch, StepStatus::Completed, None);
        } else {
            workflow.update_step(&step_create_branch, StepStatus::Skipped, None);
        }
        self.workflows.update(workflow).await?;

        workflow.update_step(&step_commit, StepStatus::Running, None);
        self.workflows.update(workflow).await?;
        if config.auto_commit {
            let commit_message = generate_commit_message(&workflow.issue, config);
            self.git.commit(&commit_message).await?;
            workflow.update_step(&step_commit, StepStatus::Completed, None);
        } else {
            workflow.update_step(&step_commit, StepStatus::Skipped, None);
        }
        self.workflows.update(workflow).await?;

        workflow.update_step(&step_push, StepStatus::Running, None);
        self.workflows.update(workflow).await?;
        if config.auto_push {
            self.git.push(&branch_name).await?;
            workflow.update_step(&step_push, StepStatus::Completed, None);
        } else {
            workflow.update_step(&step_push, StepStatus::Skipped, None);
        }
        self.workflows.update(workflow).await?;

        workflow.update_step(&step_create_pr, StepStatus::Running, None);
        self.workflows.update(workflow).await?;
        if config.auto_create_pr {
            let pr_title = generate_pr_title(&workflow.issue);
            let pr_body = generate_pr_body(&workflow.issue, config);
            let target_branch = determine_target_branch(&workflow.issue);

            let pr = self
                .github
                .create_pull_request(
                    &workflow.issue.repository,
                    &pr_title,
                    &pr_body,
                    &branch_name,
                    &target_branch,
                )
                .await?;

            let labels = extract_labels(&workflow.issue, config);
            if !labels.is_empty() {
                self.github
                    .add_labels(&workflow.issue.repository, workflow.issue.number, labels)
                    .await?;
            }

            if !config.default_reviewers.is_empty() {
                self.github
                    .add_reviewers(
                        &workflow.issue.repository,
                        pr.number,
                        config.default_reviewers.clone(),
                    )
                    .await?;
            }

            workflow.pr = Some(pr);
            workflow.update_step(&step_create_pr, StepStatus::Completed, None);
        } else {
            workflow.update_step(&step_create_pr, StepStatus::Skipped, None);
        }
        self.workflows.update(workflow).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl<G, H, W> AutomationWorkflowExecutor for ExecuteAutomationUseCase<G, H, W>
where
    G: GitOperations,
    H: GitHubClient,
    W: AutomationWorkflowRepository,
{
    async fn execute(
        &self,
        workflow: &mut AutomationWorkflow,
        config: &AutomationConfig,
    ) -> Result<(), AppError> {
        self.execute(workflow, config).await
    }

    async fn get_status(&self, workflow_id: &str) -> Result<AutomationWorkflow, AppError> {
        self.workflows.find_by_id(workflow_id).await
    }

    async fn cancel(&self, workflow_id: &str) -> Result<(), AppError> {
        let mut workflow = self.workflows.find_by_id(workflow_id).await?;
        match workflow.status {
            WorkflowStatus::Completed | WorkflowStatus::Failed | WorkflowStatus::Cancelled => {
                return Err(AppError::State(format!(
                    "workflow {workflow_id} is already {:?}",
                    workflow.status
                )));
            }
            WorkflowStatus::Pending | WorkflowStatus::InProgress => {}
        }

        workflow.status = WorkflowStatus::Cancelled;
        if let Some(step) = workflow
            .steps
            .iter_mut()
            .find(|step| matches!(step.status, StepStatus::Running | StepStatus::Pending))
        {
            step.status = StepStatus::Skipped;
        }
        self.workflows.update(&workflow).await
    }
}
