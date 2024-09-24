use leptos::{
    create_rw_signal, spawn_local, view, Fragment, IntoView, RwSignal, SignalGet, SignalSet, View,
};

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
        spawn_local(async move {
            self.apply_succeeded.set(Some(success));
            gloo_timers::future::TimeoutFuture::new(500).await;
            self.apply_succeeded.set(None);
        });
    }
}

impl IntoView for ApplyChanges {
    fn into_view(self) -> View {
        let apply_button_view = move || -> Fragment {
            match self.apply_succeeded.get() {
                None => view! {
                    Apply changes
                    <i></i>
                },
                Some(true) => view! {
                    Applied
                    <i class="fas fa-check"></i>
                },
                Some(false) => view! {
                    Error
                    <i class="fas fa-times"></i>
                },
            }
        };

        view! {
            <button
                class="submit-button"
                type="submit"
            >
                {apply_button_view}
            </button>
        }
        .into_view()
    }
}
