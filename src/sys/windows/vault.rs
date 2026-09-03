use windows::Win32::System::JobObjects::*;
use windows::Win32::System::Threading::*;
use windows::Win32::Foundation::*;

pub struct VaultSandbox {
    job_handle: HANDLE,
}

impl VaultSandbox {
    pub fn create(max_ram_mb: usize) -> Result<Self, String> {
        unsafe {
            // Create a Job Object
            let job_handle = CreateJobObjectW(None, None)
                .map_err(|e| format!("Failed to create Job Object: {}", e))?;

            // Configure basic limits (e.g., active processes)
            let mut basic_limits = JOBOBJECT_BASIC_LIMIT_INFORMATION::default();
            basic_limits.LimitFlags = JOB_OBJECT_LIMIT_ACTIVE_PROCESS | JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION;
            basic_limits.ActiveProcessLimit = 1;

            let mut ext_limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            ext_limits.BasicLimitInformation = basic_limits;
            
            // Set RAM limit
            if max_ram_mb > 0 {
                ext_limits.BasicLimitInformation.LimitFlags |= JOB_OBJECT_LIMIT_PROCESS_MEMORY;
                ext_limits.ProcessMemoryLimit = max_ram_mb * 1024 * 1024;
            }

            let limit_ptr = &ext_limits as *const _ as *const std::ffi::c_void;
            SetInformationJobObject(
                job_handle,
                JobObjectExtendedLimitInformation,
                limit_ptr,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            ).map_err(|e| format!("Failed to set memory limit: {}", e))?;

            Ok(Self { job_handle })
        }
    }

    pub fn assign_process(&self, process_id: u32) -> Result<(), String> {
        unsafe {
            let process_handle = OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, false, process_id)
                .map_err(|e| format!("Failed to open process: {}", e))?;

            AssignProcessToJobObject(self.job_handle, process_handle)
                .map_err(|e| format!("Failed to assign process to Job Object: {}", e))?;
                
            CloseHandle(process_handle).ok();
            Ok(())
        }
    }
}

impl Drop for VaultSandbox {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.job_handle).ok();
        }
    }
}
