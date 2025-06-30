# Run the Rust program and capture its output
$export_commands = cargo run --bin export_vars

# Parse and set environment variables
$export_commands -split "`n" | ForEach-Object {
    if ($_ -match '^export\s+(\w+)=(.*)$') {
        $var_name = $matches[1]
        $var_value = $matches[2]
        
        # Set environment variable in current session
        [Environment]::SetEnvironmentVariable($var_name, $var_value, "Process")
        Write-Host "Set environment variable: $var_name = $var_value"
    }
}

Write-Host "Environment variables have been exported and are now available in this PowerShell session."
Write-Host "You can verify by running: Get-ChildItem Env: | Where-Object { `$_.Name -match '^[A-Z_]+$' }" 