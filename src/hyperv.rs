//! This contains logic and types for Hyper-V
//!
use serde::Deserialize;

/// Information used to create a new virtual machine in Hyper-V 
#[derive(Debug,Deserialize)]
pub struct Vm{
    // Direct to Hyper-V
    pub hostname:   Option<String>,
    pub cpus:       Option<u8>,
    generation:     Option<Generation>,
    // Variables handled by me
    network:        Option<String>,
    os_version:     Option<Os>,

}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
enum Os {
    Win2022StandardCore,
    Win2022StandardDesktop,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
enum Generation {
    Gen1,
    Gen2,
}

/// This is the information we return to the user about the VM
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "PascalCase")]
pub struct VmStatus{
    name: String,
    state: VmState,
    id: String,
    #[serde(rename = "CPUUsage")]
    cpu_usage: u8,
    memory_assigned: u32,
    status: String,
    processor_count: u8,
    uptime: Timespan, 
}

/// C# type `System.TimeSpan` 
/// [System.TimeSpan].DeclaredFields | Select Name, FieldType
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "PascalCase")]
struct Timespan{
    days: i32,
    hours: i32,
    milliseconds: i32,
    minutes: i32,
    seconds: i32,
    ticks: i64,
    total_days: f64,
    total_hours: f64,
    total_minutes: f64,
    total_seconds: f64,
    total_milliseconds: f64,
}

/// Enum from `[System.Enum]::GetValues([Microsoft.HyperV.PowerShell.VMState])`
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
enum VmState{
    Other,
    Running,
    Off,
    Stopping,
    Saved,
    Paused,
    Starting,
    Reset,
    Saving,
    Pausing,
    Resuming,
    FastSaved,
    FastSaving,
    ForceShutdown,
    ForceReboot,
    Hibernated,
    ComponentServicing,
    RunningCritical,
    OffCritical,
    StoppingCritical,
    SavedCritical,
    PausedCritical,
    StartingCritical,
    ResetCritical,
    SavingCritical,
    PausingCritical,
    ResumingCritical,
    FastSavedCritical,
    FastSavingCritical,
}

