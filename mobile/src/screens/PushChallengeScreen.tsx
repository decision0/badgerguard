import React from 'react'
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native'

export default function PushChallengeScreen({ route }: any) {
  const { challenge } = route.params || {}

  const handleApprove = () => {
    // TODO: Send approval to backend
  }

  const handleDeny = () => {
    // TODO: Send denial to backend
  }

  return (
    <View style={styles.container}>
      <Text style={styles.title}>MFA Challenge</Text>
      {challenge && (
        <>
          <Text style={styles.label}>Application: {challenge.app_name}</Text>
          <Text style={styles.label}>IP Address: {challenge.ip_address}</Text>
          <Text style={styles.label}>Location: {challenge.country}</Text>
          <Text style={styles.label}>Time: {challenge.time}</Text>
          
          <TouchableOpacity style={styles.approveButton} onPress={handleApprove}>
            <Text style={styles.buttonText}>Approve</Text>
          </TouchableOpacity>
          
          <TouchableOpacity style={styles.denyButton} onPress={handleDeny}>
            <Text style={styles.buttonText}>Deny</Text>
          </TouchableOpacity>
        </>
      )}
    </View>
  )
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    padding: 20,
    backgroundColor: '#fff',
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 24,
  },
  label: {
    fontSize: 16,
    marginBottom: 12,
  },
  approveButton: {
    backgroundColor: '#10b981',
    padding: 16,
    borderRadius: 8,
    marginTop: 24,
  },
  denyButton: {
    backgroundColor: '#ef4444',
    padding: 16,
    borderRadius: 8,
    marginTop: 12,
  },
  buttonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
    textAlign: 'center',
  },
})

