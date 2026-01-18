local M = {}
M.blockSize = 40
M.blockDrawSize = M.blockSize - 1
M.pieceXCount = 4
M.pieceYCount = 4
M.gridXCount = 10
M.gridYCount = 18
M.offset = 5
M.timerLimit = 0.3
M.windowSize = nil

M.keys = {
	rotateLeft = "z",
	rotateRight = "x",
	hardDrop = "down",
	left = "left",
	right = "right",
	quit = "q",
	restart = "r",
}

return M
