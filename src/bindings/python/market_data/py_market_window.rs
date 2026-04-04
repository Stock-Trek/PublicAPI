#[cfg(feature = "python")]
use {
    crate::{aligned_window::AlignedWindow, rolling_window::RollingWindow},
    pyo3::{pyclass, pymethods},
};

#[cfg(feature = "python")]
#[pyclass(name = "AlignedWindow")]
pub struct PyAlignedWindow {
    inner: AlignedWindow,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyAlignedWindow {
    pub const SECONDS_5: Self = Self {
        inner: AlignedWindow::Seconds5,
    };
    pub const SECONDS_10: Self = Self {
        inner: AlignedWindow::Seconds10,
    };
    pub const SECONDS_15: Self = Self {
        inner: AlignedWindow::Seconds15,
    };
    pub const SECONDS_30: Self = Self {
        inner: AlignedWindow::Seconds30,
    };
    pub const SECONDS_60: Self = Self {
        inner: AlignedWindow::Seconds60,
    };
    pub const MINUTES_1: Self = Self {
        inner: AlignedWindow::Minutes1,
    };
    pub const MINUTES_2: Self = Self {
        inner: AlignedWindow::Minutes2,
    };
    pub const MINUTES_3: Self = Self {
        inner: AlignedWindow::Minutes3,
    };
    pub const MINUTES_5: Self = Self {
        inner: AlignedWindow::Minutes5,
    };
    pub const MINUTES_10: Self = Self {
        inner: AlignedWindow::Minutes10,
    };
    pub const MINUTES_15: Self = Self {
        inner: AlignedWindow::Minutes15,
    };
    pub const MINUTES_30: Self = Self {
        inner: AlignedWindow::Minutes30,
    };
    pub const MINUTES_60: Self = Self {
        inner: AlignedWindow::Minutes60,
    };
    pub const HOURS_1: Self = Self {
        inner: AlignedWindow::Hours1,
    };
    pub const HOURS_2: Self = Self {
        inner: AlignedWindow::Hours2,
    };
    pub const HOURS_3: Self = Self {
        inner: AlignedWindow::Hours3,
    };
    pub const HOURS_4: Self = Self {
        inner: AlignedWindow::Hours4,
    };
    pub const HOURS_6: Self = Self {
        inner: AlignedWindow::Hours6,
    };
    pub const HOURS_8: Self = Self {
        inner: AlignedWindow::Hours8,
    };
    pub const HOURS_12: Self = Self {
        inner: AlignedWindow::Hours12,
    };
    pub const HOURS_24: Self = Self {
        inner: AlignedWindow::Hours24,
    };
    pub const DAYS_1: Self = Self {
        inner: AlignedWindow::Days1,
    };
    pub const DAYS_7: Self = Self {
        inner: AlignedWindow::Days7,
    };
    pub const WEEKS_1: Self = Self {
        inner: AlignedWindow::Weeks1,
    };
    pub const WEEKS_2: Self = Self {
        inner: AlignedWindow::Weeks2,
    };
    pub const WEEKS_4: Self = Self {
        inner: AlignedWindow::Weeks4,
    };
    pub const MONTHS_1: Self = Self {
        inner: AlignedWindow::Months1,
    };
    pub const MONTHS_2: Self = Self {
        inner: AlignedWindow::Months2,
    };
    pub const MONTHS_3: Self = Self {
        inner: AlignedWindow::Months3,
    };
    pub const MONTHS_4: Self = Self {
        inner: AlignedWindow::Months4,
    };
    pub const MONTHS_6: Self = Self {
        inner: AlignedWindow::Months6,
    };
    pub const MONTHS_12: Self = Self {
        inner: AlignedWindow::Months12,
    };
    pub const YEARS_1: Self = Self {
        inner: AlignedWindow::Years1,
    };
    pub const YEARS_2: Self = Self {
        inner: AlignedWindow::Years2,
    };
    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

#[cfg(feature = "python")]
impl From<&PyAlignedWindow> for AlignedWindow {
    fn from(py: &PyAlignedWindow) -> Self {
        py.inner
    }
}

#[cfg(feature = "python")]
impl From<&AlignedWindow> for PyAlignedWindow {
    fn from(inner: &AlignedWindow) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "RollingWindow")]
pub struct PyRollingWindow {
    inner: RollingWindow,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyRollingWindow {
    pub const SECONDS_5: Self = Self {
        inner: RollingWindow::Seconds5,
    };
    pub const SECONDS_10: Self = Self {
        inner: RollingWindow::Seconds10,
    };
    pub const SECONDS_15: Self = Self {
        inner: RollingWindow::Seconds15,
    };
    pub const SECONDS_30: Self = Self {
        inner: RollingWindow::Seconds30,
    };
    pub const SECONDS_60: Self = Self {
        inner: RollingWindow::Seconds60,
    };
    pub const MINUTES_1: Self = Self {
        inner: RollingWindow::Minutes1,
    };
    pub const MINUTES_2: Self = Self {
        inner: RollingWindow::Minutes2,
    };
    pub const MINUTES_3: Self = Self {
        inner: RollingWindow::Minutes3,
    };
    pub const MINUTES_5: Self = Self {
        inner: RollingWindow::Minutes5,
    };
    pub const MINUTES_10: Self = Self {
        inner: RollingWindow::Minutes10,
    };
    pub const MINUTES_15: Self = Self {
        inner: RollingWindow::Minutes15,
    };
    pub const MINUTES_30: Self = Self {
        inner: RollingWindow::Minutes30,
    };
    pub const MINUTES_45: Self = Self {
        inner: RollingWindow::Minutes45,
    };
    pub const MINUTES_60: Self = Self {
        inner: RollingWindow::Minutes60,
    };
    pub const MINUTES_90: Self = Self {
        inner: RollingWindow::Minutes90,
    };
    pub const HOURS_1: Self = Self {
        inner: RollingWindow::Hours1,
    };
    pub const HOURS_2: Self = Self {
        inner: RollingWindow::Hours2,
    };
    pub const HOURS_3: Self = Self {
        inner: RollingWindow::Hours3,
    };
    pub const HOURS_4: Self = Self {
        inner: RollingWindow::Hours4,
    };
    pub const HOURS_6: Self = Self {
        inner: RollingWindow::Hours6,
    };
    pub const HOURS_8: Self = Self {
        inner: RollingWindow::Hours8,
    };
    pub const HOURS_12: Self = Self {
        inner: RollingWindow::Hours12,
    };
    pub const HOURS_24: Self = Self {
        inner: RollingWindow::Hours24,
    };
    pub const DAYS_1: Self = Self {
        inner: RollingWindow::Days1,
    };
    pub const DAYS_2: Self = Self {
        inner: RollingWindow::Days2,
    };
    pub const DAYS_3: Self = Self {
        inner: RollingWindow::Days3,
    };
    pub const DAYS_7: Self = Self {
        inner: RollingWindow::Days7,
    };
    pub const DAYS_14: Self = Self {
        inner: RollingWindow::Days14,
    };
    pub const DAYS_30: Self = Self {
        inner: RollingWindow::Days30,
    };
    pub const DAYS_45: Self = Self {
        inner: RollingWindow::Days45,
    };
    pub const DAYS_60: Self = Self {
        inner: RollingWindow::Days60,
    };
    pub const DAYS_90: Self = Self {
        inner: RollingWindow::Days90,
    };
    pub const DAYS_180: Self = Self {
        inner: RollingWindow::Days180,
    };
    pub const DAYS_360: Self = Self {
        inner: RollingWindow::Days360,
    };
    pub const WEEKS_1: Self = Self {
        inner: RollingWindow::Weeks1,
    };
    pub const WEEKS_2: Self = Self {
        inner: RollingWindow::Weeks2,
    };
    pub const WEEKS_4: Self = Self {
        inner: RollingWindow::Weeks4,
    };
    pub const WEEKS_10: Self = Self {
        inner: RollingWindow::Weeks10,
    };
    pub const WEEKS_20: Self = Self {
        inner: RollingWindow::Weeks20,
    };
    pub const WEEKS_40: Self = Self {
        inner: RollingWindow::Weeks40,
    };
}

#[cfg(feature = "python")]
impl From<&PyRollingWindow> for RollingWindow {
    fn from(py: &PyRollingWindow) -> Self {
        py.inner
    }
}

#[cfg(feature = "python")]
impl From<&RollingWindow> for PyRollingWindow {
    fn from(inner: &RollingWindow) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}
