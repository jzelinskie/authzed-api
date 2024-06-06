// Code generated by protoc-gen-validate. DO NOT EDIT.
// source: authzed/api/v1alpha1/watchresources_service.proto

package v1alpha1

import (
	"bytes"
	"errors"
	"fmt"
	"net"
	"net/mail"
	"net/url"
	"regexp"
	"sort"
	"strings"
	"time"
	"unicode/utf8"

	"google.golang.org/protobuf/types/known/anypb"
)

// ensure the imports are used
var (
	_ = bytes.MinRead
	_ = errors.New("")
	_ = fmt.Print
	_ = utf8.UTFMax
	_ = (*regexp.Regexp)(nil)
	_ = (*strings.Reader)(nil)
	_ = net.IPv4len
	_ = time.Duration(0)
	_ = (*url.URL)(nil)
	_ = (*mail.Address)(nil)
	_ = anypb.Any{}
	_ = sort.Sort
)

// Validate checks the field values on WatchResourcesRequest with the rules
// defined in the proto definition for this message. If any rules are
// violated, the first error encountered is returned, or nil if there are no violations.
func (m *WatchResourcesRequest) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on WatchResourcesRequest with the rules
// defined in the proto definition for this message. If any rules are
// violated, the result is a list of violation errors wrapped in
// WatchResourcesRequestMultiError, or nil if none found.
func (m *WatchResourcesRequest) ValidateAll() error {
	return m.validate(true)
}

func (m *WatchResourcesRequest) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	if len(m.GetResourceObjectType()) > 128 {
		err := WatchResourcesRequestValidationError{
			field:  "ResourceObjectType",
			reason: "value length must be at most 128 bytes",
		}
		if !all {
			return err
		}
		errors = append(errors, err)
	}

	if !_WatchResourcesRequest_ResourceObjectType_Pattern.MatchString(m.GetResourceObjectType()) {
		err := WatchResourcesRequestValidationError{
			field:  "ResourceObjectType",
			reason: "value does not match regex pattern \"^([a-z][a-z0-9_]{1,61}[a-z0-9]/)*[a-z][a-z0-9_]{1,62}[a-z0-9]$\"",
		}
		if !all {
			return err
		}
		errors = append(errors, err)
	}

	if len(m.GetPermission()) > 64 {
		err := WatchResourcesRequestValidationError{
			field:  "Permission",
			reason: "value length must be at most 64 bytes",
		}
		if !all {
			return err
		}
		errors = append(errors, err)
	}

	if !_WatchResourcesRequest_Permission_Pattern.MatchString(m.GetPermission()) {
		err := WatchResourcesRequestValidationError{
			field:  "Permission",
			reason: "value does not match regex pattern \"^[a-z][a-z0-9_]{1,62}[a-z0-9]$\"",
		}
		if !all {
			return err
		}
		errors = append(errors, err)
	}

	// no validation rules for SubjectObjectType

	// no validation rules for OptionalSubjectRelation

	if all {
		switch v := interface{}(m.GetOptionalStartCursor()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, WatchResourcesRequestValidationError{
					field:  "OptionalStartCursor",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, WatchResourcesRequestValidationError{
					field:  "OptionalStartCursor",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetOptionalStartCursor()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return WatchResourcesRequestValidationError{
				field:  "OptionalStartCursor",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	if len(errors) > 0 {
		return WatchResourcesRequestMultiError(errors)
	}

	return nil
}

// WatchResourcesRequestMultiError is an error wrapping multiple validation
// errors returned by WatchResourcesRequest.ValidateAll() if the designated
// constraints aren't met.
type WatchResourcesRequestMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m WatchResourcesRequestMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m WatchResourcesRequestMultiError) AllErrors() []error { return m }

// WatchResourcesRequestValidationError is the validation error returned by
// WatchResourcesRequest.Validate if the designated constraints aren't met.
type WatchResourcesRequestValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e WatchResourcesRequestValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e WatchResourcesRequestValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e WatchResourcesRequestValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e WatchResourcesRequestValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e WatchResourcesRequestValidationError) ErrorName() string {
	return "WatchResourcesRequestValidationError"
}

// Error satisfies the builtin error interface
func (e WatchResourcesRequestValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sWatchResourcesRequest.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = WatchResourcesRequestValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = WatchResourcesRequestValidationError{}

var _WatchResourcesRequest_ResourceObjectType_Pattern = regexp.MustCompile("^([a-z][a-z0-9_]{1,61}[a-z0-9]/)*[a-z][a-z0-9_]{1,62}[a-z0-9]$")

var _WatchResourcesRequest_Permission_Pattern = regexp.MustCompile("^[a-z][a-z0-9_]{1,62}[a-z0-9]$")

// Validate checks the field values on PermissionUpdate with the rules defined
// in the proto definition for this message. If any rules are violated, the
// first error encountered is returned, or nil if there are no violations.
func (m *PermissionUpdate) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on PermissionUpdate with the rules
// defined in the proto definition for this message. If any rules are
// violated, the result is a list of violation errors wrapped in
// PermissionUpdateMultiError, or nil if none found.
func (m *PermissionUpdate) ValidateAll() error {
	return m.validate(true)
}

func (m *PermissionUpdate) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	if all {
		switch v := interface{}(m.GetSubject()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, PermissionUpdateValidationError{
					field:  "Subject",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, PermissionUpdateValidationError{
					field:  "Subject",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetSubject()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return PermissionUpdateValidationError{
				field:  "Subject",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	if all {
		switch v := interface{}(m.GetResource()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, PermissionUpdateValidationError{
					field:  "Resource",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, PermissionUpdateValidationError{
					field:  "Resource",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetResource()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return PermissionUpdateValidationError{
				field:  "Resource",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	// no validation rules for Relation

	// no validation rules for UpdatedPermission

	if len(errors) > 0 {
		return PermissionUpdateMultiError(errors)
	}

	return nil
}

// PermissionUpdateMultiError is an error wrapping multiple validation errors
// returned by PermissionUpdate.ValidateAll() if the designated constraints
// aren't met.
type PermissionUpdateMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m PermissionUpdateMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m PermissionUpdateMultiError) AllErrors() []error { return m }

// PermissionUpdateValidationError is the validation error returned by
// PermissionUpdate.Validate if the designated constraints aren't met.
type PermissionUpdateValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e PermissionUpdateValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e PermissionUpdateValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e PermissionUpdateValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e PermissionUpdateValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e PermissionUpdateValidationError) ErrorName() string { return "PermissionUpdateValidationError" }

// Error satisfies the builtin error interface
func (e PermissionUpdateValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sPermissionUpdate.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = PermissionUpdateValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = PermissionUpdateValidationError{}

// Validate checks the field values on WatchResourcesResponse with the rules
// defined in the proto definition for this message. If any rules are
// violated, the first error encountered is returned, or nil if there are no violations.
func (m *WatchResourcesResponse) Validate() error {
	return m.validate(false)
}

// ValidateAll checks the field values on WatchResourcesResponse with the rules
// defined in the proto definition for this message. If any rules are
// violated, the result is a list of violation errors wrapped in
// WatchResourcesResponseMultiError, or nil if none found.
func (m *WatchResourcesResponse) ValidateAll() error {
	return m.validate(true)
}

func (m *WatchResourcesResponse) validate(all bool) error {
	if m == nil {
		return nil
	}

	var errors []error

	for idx, item := range m.GetUpdates() {
		_, _ = idx, item

		if all {
			switch v := interface{}(item).(type) {
			case interface{ ValidateAll() error }:
				if err := v.ValidateAll(); err != nil {
					errors = append(errors, WatchResourcesResponseValidationError{
						field:  fmt.Sprintf("Updates[%v]", idx),
						reason: "embedded message failed validation",
						cause:  err,
					})
				}
			case interface{ Validate() error }:
				if err := v.Validate(); err != nil {
					errors = append(errors, WatchResourcesResponseValidationError{
						field:  fmt.Sprintf("Updates[%v]", idx),
						reason: "embedded message failed validation",
						cause:  err,
					})
				}
			}
		} else if v, ok := interface{}(item).(interface{ Validate() error }); ok {
			if err := v.Validate(); err != nil {
				return WatchResourcesResponseValidationError{
					field:  fmt.Sprintf("Updates[%v]", idx),
					reason: "embedded message failed validation",
					cause:  err,
				}
			}
		}

	}

	if all {
		switch v := interface{}(m.GetChangesThrough()).(type) {
		case interface{ ValidateAll() error }:
			if err := v.ValidateAll(); err != nil {
				errors = append(errors, WatchResourcesResponseValidationError{
					field:  "ChangesThrough",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		case interface{ Validate() error }:
			if err := v.Validate(); err != nil {
				errors = append(errors, WatchResourcesResponseValidationError{
					field:  "ChangesThrough",
					reason: "embedded message failed validation",
					cause:  err,
				})
			}
		}
	} else if v, ok := interface{}(m.GetChangesThrough()).(interface{ Validate() error }); ok {
		if err := v.Validate(); err != nil {
			return WatchResourcesResponseValidationError{
				field:  "ChangesThrough",
				reason: "embedded message failed validation",
				cause:  err,
			}
		}
	}

	if len(errors) > 0 {
		return WatchResourcesResponseMultiError(errors)
	}

	return nil
}

// WatchResourcesResponseMultiError is an error wrapping multiple validation
// errors returned by WatchResourcesResponse.ValidateAll() if the designated
// constraints aren't met.
type WatchResourcesResponseMultiError []error

// Error returns a concatenation of all the error messages it wraps.
func (m WatchResourcesResponseMultiError) Error() string {
	var msgs []string
	for _, err := range m {
		msgs = append(msgs, err.Error())
	}
	return strings.Join(msgs, "; ")
}

// AllErrors returns a list of validation violation errors.
func (m WatchResourcesResponseMultiError) AllErrors() []error { return m }

// WatchResourcesResponseValidationError is the validation error returned by
// WatchResourcesResponse.Validate if the designated constraints aren't met.
type WatchResourcesResponseValidationError struct {
	field  string
	reason string
	cause  error
	key    bool
}

// Field function returns field value.
func (e WatchResourcesResponseValidationError) Field() string { return e.field }

// Reason function returns reason value.
func (e WatchResourcesResponseValidationError) Reason() string { return e.reason }

// Cause function returns cause value.
func (e WatchResourcesResponseValidationError) Cause() error { return e.cause }

// Key function returns key value.
func (e WatchResourcesResponseValidationError) Key() bool { return e.key }

// ErrorName returns error name.
func (e WatchResourcesResponseValidationError) ErrorName() string {
	return "WatchResourcesResponseValidationError"
}

// Error satisfies the builtin error interface
func (e WatchResourcesResponseValidationError) Error() string {
	cause := ""
	if e.cause != nil {
		cause = fmt.Sprintf(" | caused by: %v", e.cause)
	}

	key := ""
	if e.key {
		key = "key for "
	}

	return fmt.Sprintf(
		"invalid %sWatchResourcesResponse.%s: %s%s",
		key,
		e.field,
		e.reason,
		cause)
}

var _ error = WatchResourcesResponseValidationError{}

var _ interface {
	Field() string
	Reason() string
	Key() bool
	Cause() error
	ErrorName() string
} = WatchResourcesResponseValidationError{}
