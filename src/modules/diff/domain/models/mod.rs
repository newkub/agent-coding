use serde::{Deserialize, Serialize};

/// A file change representing a modification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub change_type: ChangeType,
    pub hunks: Vec<DiffHunk>,
    pub old_content: String,
    pub new_content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
}

impl FileChange {
    pub const fn new(path: String, change_type: ChangeType) -> Self {
        Self {
            path,
            change_type,
            hunks: Vec::new(),
            old_content: String::new(),
            new_content: String::new(),
        }
    }

    pub fn with_hunks(mut self, hunks: Vec<DiffHunk>) -> Self {
        self.hunks = hunks;
        self
    }

    pub fn hunk_count(&self) -> usize {
        self.hunks.len()
    }
}

/// A hunk represents a section of changes in a diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffHunk {
    pub id: String,
    pub header: String,
    pub old_start: u32,
    pub old_lines: u32,
    pub new_start: u32,
    pub new_lines: u32,
    pub lines: Vec<DiffLine>,
    pub status: HunkStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HunkStatus {
    Pending,
    Approved,
    Rejected,
}

impl DiffHunk {
    // Pure constructor - ID moved to application layer
    pub const fn create(
        id: String,
        header: String,
        old_start: u32,
        old_lines: u32,
        new_start: u32,
        new_lines: u32,
        lines: Vec<DiffLine>,
    ) -> Self {
        Self {
            id,
            header,
            old_start,
            old_lines,
            new_start,
            new_lines,
            lines,
            status: HunkStatus::Pending,
        }
    }

    pub fn approve(&mut self) {
        self.status = HunkStatus::Approved;
    }

    pub fn reject(&mut self) {
        self.status = HunkStatus::Rejected;
    }

    pub fn is_pending(&self) -> bool {
        self.status == HunkStatus::Pending
    }
}

/// A single line in a diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub content: String,
    pub old_line_num: Option<u32>,
    pub new_line_num: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffLineType {
    Context,
    Addition,
    Deletion,
    Header,
}

impl DiffLine {
    pub const fn context(content: String, old_num: u32, new_num: u32) -> Self {
        Self {
            line_type: DiffLineType::Context,
            content,
            old_line_num: Some(old_num),
            new_line_num: Some(new_num),
        }
    }

    pub const fn addition(content: String, new_num: u32) -> Self {
        Self {
            line_type: DiffLineType::Addition,
            content,
            old_line_num: None,
            new_line_num: Some(new_num),
        }
    }

    pub const fn deletion(content: String, old_num: u32) -> Self {
        Self {
            line_type: DiffLineType::Deletion,
            content,
            old_line_num: Some(old_num),
            new_line_num: None,
        }
    }
}

/// A complete diff for review
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiffReview {
    pub files: Vec<FileChange>,
    pub current_file_index: usize,
    pub current_hunk_index: usize,
    pub filter: DiffFilter,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum DiffFilter {
    #[default]
    All,
    Pending,
    Approved,
    Rejected,
}

impl DiffReview {
    pub const fn new(files: Vec<FileChange>) -> Self {
        Self {
            files,
            current_file_index: 0,
            current_hunk_index: 0,
            filter: DiffFilter::All,
        }
    }

    pub fn current_file(&self) -> Option<&FileChange> {
        self.files.get(self.current_file_index)
    }

    pub fn current_hunk(&self) -> Option<&DiffHunk> {
        self.current_file()?.hunks.get(self.current_hunk_index)
    }

    pub fn next_hunk(&mut self) -> bool {
        if let Some(file) = self.files.get_mut(self.current_file_index) {
            if self.current_hunk_index < file.hunks.len() - 1 {
                self.current_hunk_index += 1;
                return true;
            }
        }
        
        // Move to next file
        if self.current_file_index < self.files.len() - 1 {
            self.current_file_index += 1;
            self.current_hunk_index = 0;
            return true;
        }
        false
    }

    pub fn prev_hunk(&mut self) -> bool {
        if self.current_hunk_index > 0 {
            self.current_hunk_index -= 1;
            return true;
        }
        
        if self.current_file_index > 0 {
            self.current_file_index -= 1;
            self.current_hunk_index = self.files[self.current_file_index]
                .hunks
                .len()
                .saturating_sub(1);
            return true;
        }
        false
    }

    pub fn approve_current_hunk(&mut self) {
        if let Some(file) = self.files.get_mut(self.current_file_index) {
            if let Some(hunk) = file.hunks.get_mut(self.current_hunk_index) {
                hunk.approve();
            }
        }
    }

    pub fn reject_current_hunk(&mut self) {
        if let Some(file) = self.files.get_mut(self.current_file_index) {
            if let Some(hunk) = file.hunks.get_mut(self.current_hunk_index) {
                hunk.reject();
            }
        }
    }

    pub fn approved_count(&self) -> usize {
        self.files
            .iter()
            .flat_map(|f| &f.hunks)
            .filter(|h| h.status == HunkStatus::Approved)
            .count()
    }

    pub fn pending_count(&self) -> usize {
        self.files
            .iter()
            .flat_map(|f| &f.hunks)
            .filter(|h| h.status == HunkStatus::Pending)
            .count()
    }
}