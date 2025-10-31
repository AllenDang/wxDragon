use std::fmt;

/// Comprehensive error types for virtual list operations
#[derive(Debug, Clone)]
pub enum VirtualListError {
    /// Invalid item index provided
    InvalidIndex { index: usize, total_items: usize },

    /// Measurement operation failed
    MeasurementFailed { index: usize, reason: String },

    /// Invalid configuration provided
    InvalidConfig { message: String },

    /// Panel operation failed
    PanelOperationFailed { operation: String, details: Option<String> },

    /// Data source operation failed
    DataSourceError { message: String },

    /// Item renderer operation failed
    RendererError {
        index: usize,
        operation: String,
        details: Option<String>,
    },

    /// Pool operation failed
    PoolError { operation: String, reason: String },

    /// Cache operation failed
    CacheError { operation: String, reason: String },

    /// Layout calculation failed
    LayoutError { operation: String, details: String },

    /// Resource allocation failed
    ResourceError { resource: String, reason: String },
}

impl fmt::Display for VirtualListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VirtualListError::InvalidIndex { index, total_items } => {
                write!(f, "Invalid item index: {index} (total items: {total_items})")
            }
            VirtualListError::MeasurementFailed { index, reason } => {
                write!(f, "Measurement failed for item {index}: {reason}")
            }
            VirtualListError::InvalidConfig { message } => {
                write!(f, "Invalid configuration: {message}")
            }
            VirtualListError::PanelOperationFailed { operation, details } => match details {
                Some(details) => write!(f, "Panel operation '{operation}' failed: {details}"),
                None => write!(f, "Panel operation '{operation}' failed"),
            },
            VirtualListError::DataSourceError { message } => {
                write!(f, "Data source error: {message}")
            }
            VirtualListError::RendererError {
                index,
                operation,
                details,
            } => match details {
                Some(details) => write!(f, "Renderer operation '{operation}' failed for item {index}: {details}"),
                None => write!(f, "Renderer operation '{operation}' failed for item {index}"),
            },
            VirtualListError::PoolError { operation, reason } => {
                write!(f, "Pool operation '{operation}' failed: {reason}")
            }
            VirtualListError::CacheError { operation, reason } => {
                write!(f, "Cache operation '{operation}' failed: {reason}")
            }
            VirtualListError::LayoutError { operation, details } => {
                write!(f, "Layout operation '{operation}' failed: {details}")
            }
            VirtualListError::ResourceError { resource, reason } => {
                write!(f, "Resource allocation failed for '{resource}': {reason}")
            }
        }
    }
}

impl std::error::Error for VirtualListError {}

/// Result type for virtual list operations
pub type VirtualListResult<T> = Result<T, VirtualListError>;

/// Helper methods for creating common errors
impl VirtualListError {
    /// Create an invalid index error
    pub fn invalid_index(index: usize, total_items: usize) -> Self {
        Self::InvalidIndex { index, total_items }
    }

    /// Create a measurement failed error
    pub fn measurement_failed(index: usize, reason: impl Into<String>) -> Self {
        Self::MeasurementFailed {
            index,
            reason: reason.into(),
        }
    }

    /// Create an invalid config error
    pub fn invalid_config(message: impl Into<String>) -> Self {
        Self::InvalidConfig { message: message.into() }
    }

    /// Create a panel operation failed error
    pub fn panel_operation_failed(operation: impl Into<String>, details: Option<impl Into<String>>) -> Self {
        Self::PanelOperationFailed {
            operation: operation.into(),
            details: details.map(|d| d.into()),
        }
    }

    /// Create a data source error
    pub fn data_source_error(message: impl Into<String>) -> Self {
        Self::DataSourceError { message: message.into() }
    }

    /// Create a renderer error
    pub fn renderer_error(index: usize, operation: impl Into<String>, details: Option<impl Into<String>>) -> Self {
        Self::RendererError {
            index,
            operation: operation.into(),
            details: details.map(|d| d.into()),
        }
    }

    /// Create a pool error
    pub fn pool_error(operation: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::PoolError {
            operation: operation.into(),
            reason: reason.into(),
        }
    }

    /// Create a cache error
    pub fn cache_error(operation: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::CacheError {
            operation: operation.into(),
            reason: reason.into(),
        }
    }

    /// Create a layout error
    pub fn layout_error(operation: impl Into<String>, details: impl Into<String>) -> Self {
        Self::LayoutError {
            operation: operation.into(),
            details: details.into(),
        }
    }

    /// Create a resource error
    pub fn resource_error(resource: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ResourceError {
            resource: resource.into(),
            reason: reason.into(),
        }
    }
}

/// Helper trait for converting common error patterns to VirtualListError
pub trait IntoVirtualListError<T> {
    /// Convert to VirtualListResult with context
    fn into_vl_error(self, operation: impl Into<String>) -> VirtualListResult<T>;
}

impl<T> IntoVirtualListError<T> for Option<T> {
    fn into_vl_error(self, operation: impl Into<String>) -> VirtualListResult<T> {
        self.ok_or_else(|| VirtualListError::panel_operation_failed(operation, Some("Operation returned None".to_string())))
    }
}

/// Error context helper for adding operation context to errors
pub trait VirtualListErrorContext<T> {
    /// Add context to an error
    fn with_context(self, context: impl Into<String>) -> VirtualListResult<T>;
}

impl<T, E> VirtualListErrorContext<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn with_context(self, context: impl Into<String>) -> VirtualListResult<T> {
        self.map_err(|e| VirtualListError::panel_operation_failed(context, Some(e.to_string())))
    }
}
