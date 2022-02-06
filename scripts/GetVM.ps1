#!/usr/bin/pwsh
Param(
	[string]$vmname
)

$vms = invoke-Command `
	-hostname bot@BestPCGargunnock `
	-command { Get-VM }

$out = $vms | Where-Object VMName -eq $vmname | `
	Select-Object Name, Id, CPUUsage, MemoryAssigned, uptime, `
	Status, State, ProcessorCount | ConvertTo-JSON -Compress -EnumsAsStrings


if ($out){
	write-output $out
	exit 0
}else{
	write-error `
		-message "VM ``$($vmname)`` cannot be found" `
		-category InvalidArgument `
		-recommendedAction "Check that the VM exists"
	exit 1
}
