function randomString(length)
    math.randomseed(socket.gettime()*10000)
    local res = ""
    for i = 1, length do res = res .. string.char(math.random(97, 122)) end
    return res
end

function new()
    return randomString()
end

function sleep(seconds)
    os.execute("sleep " .. tonumber(seconds))
end