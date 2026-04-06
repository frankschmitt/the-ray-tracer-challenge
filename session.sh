cd /home/frank/src/the-ray-tracer-challenge
# vim session with source files
tmux "new-session" -s "raytracer" -d
tmux send-keys -t aoc 'vim src/*.rs README.adoc' C-m
tmux "split-window" -h
tmux attach -t "raytracer"

