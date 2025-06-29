use crate::{Error, ServiceConfig, ServiceStatus, StartType};
use std::ffi::c_void;
use std::path::Path;
use windows::core::{HSTRING, PWSTR};
use windows::Win32::Foundation::{CloseHandle, HANDLE, WIN32_ERROR};
use windows::Win32::System::Services::{
    ChangeServiceConfig2W, ControlService, CreateServiceW, DeleteService, OpenSCManagerW,
    OpenServiceW, QueryServiceStatusEx, StartServiceW, SC_HANDLE, SC_MANAGER_ALL_ACCESS,
    SC_STATUS_PROCESS_INFO, SERVICE_ALL_ACCESS, SERVICE_AUTO_START, SERVICE_CONFIG_DESCRIPTION,
    SERVICE_CONTROL_STOP, SERVICE_DEMAND_START, SERVICE_DESCRIPTIONW, SERVICE_DISABLED,
    SERVICE_ERROR_NORMAL, SERVICE_STATUS, SERVICE_STATUS_PROCESS, SERVICE_WIN32_OWN_PROCESS,
};

pub struct ServiceManager {
    sc_manager: SC_HANDLE,
    service: Option<SC_HANDLE>,
    service_name: HSTRING,
}

impl ServiceManager {
    pub fn new(service_name: &str) -> Result<Self, Error> {
        let sc_manager = unsafe {
            OpenSCManagerW(None, None, SC_MANAGER_ALL_ACCESS)
                .map_err(|e| Error::Windows(WIN32_ERROR(e.code().0 as u32)))?
        };
        Ok(ServiceManager {
            sc_manager,
            service: None,
            service_name: HSTRING::from(service_name),
        })
    }

    pub fn install(&mut self, config: &ServiceConfig) -> Result<(), Error> {
        if !Path::new(&config.exe_path).exists() {
            return Err(Error::InvalidPath(config.exe_path.clone()));
        }

        let start_type = match config.start_type {
            StartType::Auto => SERVICE_AUTO_START,
            StartType::Manual | StartType::Demand => SERVICE_DEMAND_START,
            StartType::Disabled => SERVICE_DISABLED,
        };

        let service = unsafe {
            CreateServiceW(
                self.sc_manager,
                &self.service_name,
                &HSTRING::from(&config.display_name),
                SERVICE_ALL_ACCESS,
                SERVICE_WIN32_OWN_PROCESS,
                start_type,
                SERVICE_ERROR_NORMAL,
                &HSTRING::from(&config.exe_path),
                None,
                None,
                None,
                None,
                None,
            )
            .map_err(|e| Error::Windows(WIN32_ERROR(e.code().0 as u32)))?
        };

        self.service = Some(service);
        if !config.description.is_empty() {
            let desc_str = HSTRING::from(&config.description);
            let desc = SERVICE_DESCRIPTIONW {
                lpDescription: PWSTR(desc_str.as_ptr() as *mut _),
            };

            unsafe {
                ChangeServiceConfig2W(
                    service,
                    SERVICE_CONFIG_DESCRIPTION,
                    Some(&desc as *const _ as *const c_void),
                )
                .map_err(|e| Error::Windows(WIN32_ERROR(e.code().0 as u32)))?;
            }
        }

        Ok(())
    }

    pub fn uninstall(&mut self) -> Result<(), Error> {
        self.open_service()?;
        if let Some(service) = self.service {
            unsafe {
                DeleteService(service)
                    .map_err(|e| Error::Windows(WIN32_ERROR(e.code().0 as u32)))?;
            }
            self.service = None;
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Error> {
        self.open_service()?;
        if let Some(service) = self.service {
            unsafe {
                StartServiceW(service, None)
                    .map_err(|e| Error::Windows(WIN32_ERROR(e.code().0 as u32)))?;
            }
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        self.open_service()?;
        if let Some(service) = self.service {
            let mut status = SERVICE_STATUS::default();
            unsafe {
                ControlService(service, SERVICE_CONTROL_STOP, &mut status)
                    .map_err(|e| Error::Windows(WIN32_ERROR(e.code().0 as u32)))?;
            }
        }
        Ok(())
    }

    pub fn status(&mut self) -> Result<ServiceStatus, Error> {
        self.open_service()?;
        if let Some(service) = self.service {
            let mut status = SERVICE_STATUS_PROCESS::default();
            let mut bytes_needed = 0;
            unsafe {
                let buffer = std::slice::from_raw_parts_mut(
                    &mut status as *mut _ as *mut u8,
                    size_of::<SERVICE_STATUS_PROCESS>(),
                );

                QueryServiceStatusEx(
                    service,
                    SC_STATUS_PROCESS_INFO,
                    Some(buffer),
                    &mut bytes_needed,
                )
                .map_err(|e| Error::Windows(WIN32_ERROR(e.code().0 as u32)))?;
            }
            Ok(ServiceStatus::from(status.dwCurrentState.0))
        } else {
            Ok(ServiceStatus::Stopped)
        }
    }

    fn open_service(&mut self) -> Result<(), Error> {
        if self.service.is_none() {
            let service = unsafe {
                OpenServiceW(self.sc_manager, &self.service_name, SERVICE_ALL_ACCESS)
                    .map_err(|e| Error::Windows(WIN32_ERROR(e.code().0 as u32)))?
            };
            self.service = Some(service);
        }
        Ok(())
    }
}

impl Drop for ServiceManager {
    fn drop(&mut self) {
        if let Some(service) = self.service.take() {
            if !service.is_invalid() {
                unsafe {
                    let _ = CloseHandle(HANDLE(service.0));
                }
            }
        }
        if !self.sc_manager.is_invalid() {
            unsafe {
                let _ = CloseHandle(HANDLE(self.sc_manager.0));
            }
        }
    }
}
