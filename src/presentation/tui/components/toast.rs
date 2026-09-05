#![allow(dead_code)]
use std::collections::VecDeque;

use ratatui::{layout::Rect, text::Line, Frame};
use ratatui_ui::{Toast, ToastLevel, ToastManager, ToastPosition};

use super::styles::*;
use crate::modules::ui::domain::models::{AppState, ToastKind, ToastNotification};

fn to_level(kind: ToastKind) -> ToastLevel {
    match kind {
        ToastKind::Info => ToastLevel::Info,
        ToastKind::Success => ToastLevel::Success,
        ToastKind::Warning => ToastLevel::Warning,
        ToastKind::Error => ToastLevel::Error,
    }
}

pub(crate) fn draw_toasts(frame: &mut Frame, area: Rect, state: &AppState) {
    if state.toasts.is_empty() {
        return;
    }

    let theme = rt_theme();
    let toasts: Vec<Toast<'_>> = state
        .toasts
        .iter()
        .map(|toast| {
            Toast::new(
                Line::raw(toast.message.clone()),
                to_level(toast.kind),
                &theme,
            )
        })
        .collect();

    ToastManager::new(&theme)
        .toasts(toasts)
        .position(ToastPosition::BottomRight)
        .width(40)
        .render(area, frame.buffer_mut());
}

pub(crate) fn draw_toast_overlay(
    frame: &mut Frame,
    area: Rect,
    toasts: &VecDeque<ToastNotification>,
) {
    if toasts.is_empty() {
        return;
    }

    let theme = rt_theme();
    if let Some(toast) = toasts.front() {
        let t = Toast::new(
            Line::raw(toast.message.clone()),
            to_level(toast.kind),
            &theme,
        );
        ToastManager::new(&theme)
            .toasts(vec![t])
            .position(ToastPosition::BottomLeft)
            .width(40)
            .render(area, frame.buffer_mut());
    }
}
