timestamp=$(date "+%Y-%m-%d %H:%M:%S")
cd ~/lab/website && git add -A && git commit -m "Auto commit at $timestamp" && git push origin master
