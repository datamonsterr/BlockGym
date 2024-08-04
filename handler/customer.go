package handler

import (
	viewCommon "github.com/datamonsterr/PTSolana/view/common"
	"github.com/labstack/echo/v4"
)

func GetUser(c echo.Context) error {
	return RenderTemplComp(c, viewCommon.User(c.Param("id")))
}
