package commands

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

// isZero returns true if the passed value is the zero object
func isZero(v reflect.Value) bool {
	switch v.Kind() {
	case reflect.Slice, reflect.Map:
		return v.Len() == 0
	}

	// compare the types directly with more general coverage
	return reflect.DeepEqual(v.Interface(), reflect.Zero(v.Type()).Interface())
}