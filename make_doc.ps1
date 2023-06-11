cargo doc --no-deps
Remove-Item -ErrorAction Ignore -Recurse -Force ./docs
Write-Output '<meta http-equiv="refresh" content="0; url=headache">' | Out-File -append -encoding utf8 target/doc/index.html
Copy-Item -Recurse target/doc ./docs