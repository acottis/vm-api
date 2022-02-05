#!/usr/bin/pwsh

Param(
	[String]$vmname
)

$vms = Invoke-Command `
	-HostName bot@BestPCGargunnock `
	-Command { Get-VM }

$out = $vms | Where-Object VMName -eq $vmname | `
Select-Object Name, Id, CPUUsage, MemoryAssigned, Uptime, Status, State, ProcessorCount | ConvertTo-JSON -Compress


if ($out){
	Write-Output $out
	exit 0
}else{
	Write-Error `
		-Message "VM ``$($vmname)`` cannot be found" `
		-Category InvalidArgument `
		-RecommendedAction "Check that the VM exists"
	exit 1
}
	
