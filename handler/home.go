package handler

import (
	"github.com/brianhuster/PTSolana/view"
	"github.com/labstack/echo/v4"
)

func GetIndex(c echo.Context) error {
	return RenderTemplComp(c, view.Index())
}
