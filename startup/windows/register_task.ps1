$Action = New-ScheduledTaskAction -Execute "C:\path\to\vault_sync.exe"
$Trigger = New-ScheduledTaskTrigger -AtStartup
$Principal = New-ScheduledTaskPrincipal -UserId "$env:USERNAME" -RunLevel Highest
Register-ScheduledTask -TaskName "VaultSync" -Action $Action -Trigger $Trigger -Principal $Principal
