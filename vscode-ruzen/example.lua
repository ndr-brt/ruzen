for i=1,32 do 
    inst("aaa", "sine", {})
    inst("bbb", "kick", {})
    sleep(0.1)
end

inst("bbb", "kick", {})

p(1, "~ kick ~ kick")
p(2, "snare ~ snare")
p(3, "~")

hush()
start()

for i=1,10 do 
    play("kick")
    sleep(0.05)
    play("snare")
    sleep(0.15)
    play("catta")
    sleep(0.15)
    play("kick")
    sleep(0.10)
end