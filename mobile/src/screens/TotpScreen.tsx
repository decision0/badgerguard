import React from 'react'
import { View, Text, StyleSheet } from 'react-native'

export default function TotpScreen() {
  return (
    <View style={styles.container}>
      <Text style={styles.title}>TOTP Tokens</Text>
      <Text style={styles.subtitle}>No TOTP tokens configured</Text>
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
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 16,
    color: '#666',
  },
})

