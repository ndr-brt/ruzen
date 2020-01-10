function sleep(seconds)
    os.execute("sleep " .. tonumber(seconds))
end

for i=1,10 do 
    play("kick") 
    sleep(1/i)
end

for i=1,10 do 
    play("snare") 
    sleep(1/i)
end