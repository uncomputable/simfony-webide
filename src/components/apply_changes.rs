use leptos::{create_rw_signal, view, IntoView, RwSignal, SignalGet, SignalSet, View};

#[derive(Copy, Clone, Debug)]
pub struct ApplyChanges {
    apply_succeeded: RwSignal<Option<bool>>,
}

impl Default for ApplyChanges {
    fn default() -> Self {
        Self {
            apply_succeeded: create_rw_signal(None),
        }
    }
}

impl ApplyChanges {
    pub fn set_success(self, success: bool) {
        self.apply_succeeded.set(Some(success));
    }
}

impl IntoView for ApplyChanges {
    fn into_view(self) -> View {
        let tooltip_text = move || -> &'static str {
            match self.apply_succeeded.get() {
                None => "Apply",
                Some(true) => "Applied!",
                Some(false) => "Error!",
            }
        };

        view! {
            <div class="tooltip">
                <button
                    class="submit-button"
                    type="submit"
                    on:mouseout=move |_| self.apply_succeeded.set(None)
                >
                    <span class="tooltip-text">{tooltip_text}</span>
                    <i class="fas fa-floppy-disk"></i>
                    Apply changes
                </button>
            </div>
        }
        .into_view()
    }
}
