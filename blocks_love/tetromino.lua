---@class Tetromino
---@field x integer
---@field y integer
---@field type integer
---@field rotation integer
local tetromino = {}
tetromino.__index = tetromino

local conf = require("config")
local shapes = require("shapes")

function tetromino.new(type)
	local self = setmetatable({}, tetromino)
	self.x = 3
	self.y = 0
	self.type = type
	self.rotation = 1
	return self
end


---comment
---@param tetro Tetromino
function tetromino.clone(tetro)
    local clone = setmetatable({}, tetromino)
    clone.x = tetro.x
    clone.y = tetro.y
    clone.type = tetro.type
    clone.rotation = tetro.rotation
    return clone
end

return tetromino
