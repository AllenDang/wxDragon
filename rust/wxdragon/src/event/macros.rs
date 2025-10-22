//! Macros for implementing event handling in widgets.

/// Generates internal binding method and public on_* methods for window events
#[macro_export]
macro_rules! implement_window_event_handlers {
    ($widget:ident,
     $($event_variant:ident => $method_name:ident, $event_type:expr, $event_data:ident),+) => {
        impl $widget {
            // Internal binding method
            #[doc(hidden)]
            pub(crate) fn bind_window_event<F>(&self, event_type: $crate::event::WindowEvent, mut callback: F) -> $crate::event::EventToken
            where
                F: FnMut($crate::event::WindowEventData) + 'static
            {
                // Map WindowEvent to appropriate EventType
                let event_type_ffi = match event_type {
                    $($crate::event::WindowEvent::$event_variant => $event_type,)*
                };

                // Convert raw event to typed event
                let wrapper = move |event: $crate::event::Event| {
                    let typed_event = match event_type {
                        $($crate::event::WindowEvent::$event_variant =>
                            $crate::event::WindowEventData::$event_data($crate::event::$event_data::new(event)),)*
                        _ => $crate::event::WindowEventData::General(event),
                    };

                    callback(typed_event);
                };

                // Use internal bind method and return token
                $crate::event::WxEvtHandler::bind_internal(self, event_type_ffi, wrapper)
            }

            // Generate public on_* methods
            $(
                paste::paste! {
                    /// Binds a handler to a window event.
                    /// Returns an EventToken that can be used to unbind the handler later.
                    pub fn [<on_ $method_name>]<F>(&self, mut callback: F) -> $crate::event::EventToken
                    where
                        F: FnMut($crate::event::[<$event_data Event>]) + 'static
                    {
                        self.bind_window_event($crate::event::WindowEvent::$event_variant, move |event| {
                            if let $crate::event::WindowEventData::$event_data(typed_event) = event {
                                callback(typed_event);
                            }
                        })
                    }
                }
            )*
        }
    }
}

/// Generates internal binding method and public on_* methods for category-specific events
#[macro_export]
macro_rules! implement_category_event_handlers {
    // Generic implementation for category event traits
    ($trait_name:ident, $event_enum:ident, $event_data:ident,
     $($variant:ident => $method_name:ident, $event_type:expr),+) => {
        pub trait $trait_name: $crate::event::WxEvtHandler {
            // Internal binding method
            #[doc(hidden)]
            fn bind_category_event<F>(&self, event: $crate::event::$event_enum, mut callback: F) -> $crate::event::EventToken
            where
                F: FnMut($crate::event::$event_data) + 'static
            {
                // Map enum variant to EventType
                let event_type = match event {
                    $($crate::event::$event_enum::$variant => $event_type,)*
                };

                // Create wrapper
                let wrapper = move |event: $crate::event::Event| {
                    let typed_event = $crate::event::$event_data::new(event);
                    callback(typed_event);
                };

                // Use internal bind method and return token
                $crate::event::WxEvtHandler::bind_internal(self, event_type, wrapper)
            }

            // Public helper methods
            $(
                paste::paste! {
                    /// Binds a handler to a category-specific event.
                    /// Returns an EventToken that can be used to unbind the handler later.
                    fn [<on_ $method_name>]<F>(&self, callback: F) -> $crate::event::EventToken
                    where
                        F: FnMut($crate::event::$event_data) + 'static
                    {
                        self.bind_category_event($crate::event::$event_enum::$variant, callback)
                    }
                }
            )*
        }
    }
}

/// Generates internal binding method and public on_* methods for widget-specific events defined in the same module
#[macro_export]
macro_rules! implement_widget_local_event_handlers {
    ($widget:ident, $event_enum:ident, $event_data:ident,
     $($variant:ident => $method_name:ident, $event_type:expr),+) => {
        impl $widget {
            // Internal binding method
            #[doc(hidden)]
            pub(crate) fn bind_widget_event<F>(&self, event: $event_enum, mut callback: F) -> $crate::event::EventToken
            where
                F: FnMut($event_data) + 'static
            {
                // Map enum variant to EventType
                let event_type = match event {
                    $($event_enum::$variant => $event_type,)*
                };

                // Create wrapper
                let wrapper = move |event: $crate::event::Event| {
                    let typed_event = $event_data::new(event);
                    callback(typed_event);
                };

                // Use internal bind method and return token
                $crate::event::WxEvtHandler::bind_internal(self, event_type, wrapper)
            }

            // Public helper methods
            $(
                paste::paste! {
                    /// Binds a handler to a widget-specific event.
                    /// Returns an EventToken that can be used to unbind the handler later.
                    pub fn [<on_ $method_name>]<F>(&self, callback: F) -> $crate::event::EventToken
                    where
                        F: FnMut($event_data) + 'static
                    {
                        self.bind_widget_event($event_enum::$variant, callback)
                    }
                }
            )*
        }
    }
}
