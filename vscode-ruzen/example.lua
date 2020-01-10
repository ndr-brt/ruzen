function sleep(seconds)
    os.execute("sleep " .. tonumber(seconds))
end

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

start()

print(coroutine.status(sn))