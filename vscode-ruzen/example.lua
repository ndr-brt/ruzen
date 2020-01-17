

ki = coroutine.create(function ()
    for i=1,10 do 
        play("kick") 
        sleep(1/i)
    end
end)

sn = coroutine.create(function ()
    for i=1,10 do 
        play("snare") 
        sleep(1/i)
    end
end)

function start()
    coroutine.resume(ki)
    coroutine.resume(sn)
end

for i=1,32 do 
    inst("aaa", "sine", {})
    inst("bbb", "kick", {})
    sleep(0.1)
end

inst("bbb", "kick", {})

p(1, "kick snare kick")
p(2, "snare")
p(3, "catta")

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