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

function tetromino:canMove(direction_x, direction_y, testRotation, grid)
	local testX = self.x + direction_x
	local testY = self.y + direction_y

	for y = 1, conf.pieceYCount do
		for x = 1, conf.pieceXCount do
			local testBlockX = testX + x
			local testBlockY = testY + y
			if
				shapes.pieceStructures[self.type][testRotation][y][x] ~= " "
				and (
					testBlockX < 1
					or testBlockX > conf.gridXCount
					or testBlockY > conf.gridYCount
					or grid[testBlockY][testBlockX] ~= " "
				)
			then
				return false
			end
		end
	end
	return true
end

return tetromino
