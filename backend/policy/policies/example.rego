package badgerguard.policy

import future.keywords.if
import future.keywords.in

# Default deny
default allow := false

# Allow access if all conditions are met
allow if {
    # User is authenticated
    input.user_id != ""
    
    # Check time-based access (example: 08:00-20:00)
    time_allowed
    
    # Check geo restrictions (example: only SG)
    geo_allowed
    
    # Check device posture
    device_allowed
}

# Time-based access control
time_allowed if {
    hour := time.clock(input.time_of_day)[0]
    hour >= 8
    hour < 20
}

# Geo-based access control
geo_allowed if {
    input.country == "SG"
}

# Device-based access control
device_allowed if {
    input.device_id != ""
    # TODO: Add device trust check
}

# Determine required MFA methods
required_mfa contains "webauthn" if {
    input.sensitivity == "high"
}

required_mfa contains "totp" if {
    input.sensitivity == "medium"
}

