#[cfg(target_os = "windows")]
pub mod windows {
    use windows::{
        core::*,
        Win32::Media::Audio::Endpoints::IAudioEndpointVolume,
        Win32::Media::Audio::{eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator},
        Win32::System::Com::{
            CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
        },
    };

    pub struct VolumeControl;

    impl VolumeControl {
        /// Initialize COM and get the audio endpoint volume interface
        fn get_endpoint_volume() -> Result<IAudioEndpointVolume> {
            unsafe {
                // Initialize COM (ignore error if already initialized)
                let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

                // Create device enumerator
                let enumerator: IMMDeviceEnumerator =
                    CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

                // Get default audio endpoint
                let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;

                // Activate the audio endpoint volume interface
                let endpoint_volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;

                Ok(endpoint_volume)
            }
        }

        /// Get the current master volume level (0-100)
        pub fn get_volume() -> Result<i32> {
            unsafe {
                let endpoint_volume = Self::get_endpoint_volume()?;
                let volume_scalar = endpoint_volume.GetMasterVolumeLevelScalar()?;

                // Clean up COM
                CoUninitialize();

                // Convert to percentage (0-100)
                Ok((volume_scalar * 100.0).round() as i32)
            }
        }

        /// Set the master volume level (0-100)
        pub fn set_volume(level: i32) -> Result<()> {
            unsafe {
                let endpoint_volume = Self::get_endpoint_volume()?;

                // Clamp to 0-100 and convert to scalar (0.0-1.0)
                let clamped = level.clamp(0, 100);
                let scalar = clamped as f32 / 100.0;

                endpoint_volume.SetMasterVolumeLevelScalar(scalar, std::ptr::null())?;

                // Clean up COM
                CoUninitialize();

                Ok(())
            }
        }

        /// Increase volume by a specific amount
        pub fn increase_volume(amount: i32) -> Result<i32> {
            let current = Self::get_volume()?;
            let new_volume = (current + amount).min(100);
            Self::set_volume(new_volume)?;
            Ok(new_volume)
        }

        /// Decrease volume by a specific amount
        pub fn decrease_volume(amount: i32) -> Result<i32> {
            let current = Self::get_volume()?;
            let new_volume = (current - amount).max(0);
            Self::set_volume(new_volume)?;
            Ok(new_volume)
        }
    }
}

// Re-export for easier access
#[cfg(target_os = "windows")]
pub use windows::VolumeControl;
