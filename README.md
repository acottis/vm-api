# This goes with order 65
This is an API endpoint to manage hyper-v and deploy new VM's

## Usage
Root endpoint is http://vmapi:8080/api/v0/
Endpoints:
* new - POST
* status - GET

## TODO
This is currently a Minimum viable project so a lot. We only currently only have 2 endpoints which handles the creation of a VM and status of a VM. Could add a GUI and options for multiple VMs creation, deletion and also could handle more operations for VM management.
Also need much better error handling on exisiting endpoints with guidence to user on failure, every invalid entry should push the user towards to correct answer
Better logging for the server log for troubleshooting

## Examples
```Powershell
Invoke-WebRequest -Uri http://vmapi:8080/api/v0/status?hostname=NewVM -Method GET
```

```Powershell
$body = @{ 
	hostname="HOME-DC"
	cpus=8
	generation=Gen2
	os_version=Win2022StandardDesktop
} | ConvertTo-Json
Invoke-WebRequest -Uri http://vmapi:8080/api/v0/new -Method POST -Body $body -ContentType 'application/json'
```
