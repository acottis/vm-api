Param(
	[Parameter(Mandatory)]
	[String]$VMName
)

Stop-VM -name $VMName -force; Remove-VM -Name $VMName -force; Remove-Item -Path "D:\HyperV\$($VMName)\" -Recurse -force
