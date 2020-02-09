package targets

import (
	"errors"
	"reflect"
	"strconv"
)

func isInt(val interface{}) error {
	value := reflect.ValueOf(val)

	switch value.Kind() {
	case reflect.Int:
		return nil
	case reflect.String:
		_, err := strconv.Atoi(value.String())
		if err != nil {
			return errors.New("value must int")
		}
	default:
		return errors.New("value must int")
	}

	return nil
}
