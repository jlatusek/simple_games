local conf = require("config")

local M = {}

local rotation_matrix = {
	-- 0
	{
		{ 1, 0 },
		{ 0, 1 },
	},
	-- 90
	{
		{ 0, -1 },
		{ 1, 0 },
	},
	-- 180
	{
		{ -1, 0 },
		{ 0, -1 },
	},
	-- 270
	{
		{ 0, 1 },
		{ -1, 0 },
	},
}

-- Pre-calculate offsets to avoid doing division every frame
local xOffset = (conf.gridXCount / 2) + 1
local yOffset = (conf.gridYCount / 2) + 1

---Converts centered coordinates to table indices
---@param x integer The centered X coordinate (e.g. -5)
---@param y integer The centered Y coordinate (e.g. 0)
---@return integer ix The internal array X index (1-based)
---@return integer iy The internal array Y index (1-based)
function M.gridToTableIndex(x, y)
	local ix = x + xOffset
	local iy = y + yOffset
	return ix, iy
end

function M.tableToGridIndex(x, y)
	local ix = x - xOffset
	local iy = y - yOffset
	return ix, iy
end

function M.rotate(x, y, rotation)
	local rot = rotation_matrix[rotation]
	local ix = rot[1][1] * x - rot[1][2] * y
	local iy = rot[2][1] * x + rot[2][2] * y
	return ix, iy
end

return M
