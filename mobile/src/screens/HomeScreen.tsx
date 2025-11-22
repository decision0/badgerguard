import React from 'react'
import { View, Text, StyleSheet, TouchableOpacity } from 'react-native'

export default function HomeScreen({ navigation }: any) {
  return (
    <View style={styles.container}>
      <Text style={styles.title}>BadgerGuard</Text>
      <Text style={styles.subtitle}>Zero-Trust MFA</Text>
      
      <TouchableOpacity
        style={styles.button}
        onPress={() => navigation.navigate('Totp')}
      >
        <Text style={styles.buttonText}>TOTP Tokens</Text>
      </TouchableOpacity>
      
      <TouchableOpacity
        style={styles.button}
        onPress={() => navigation.navigate('Settings')}
      >
        <Text style={styles.buttonText}>Settings</Text>
      </TouchableOpacity>
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
    fontSize: 32,
    fontWeight: 'bold',
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 18,
    color: '#666',
    marginBottom: 32,
  },
  button: {
    backgroundColor: '#0284c7',
    padding: 16,
    borderRadius: 8,
    marginBottom: 12,
  },
  buttonText: {
    color: '#fff',
    fontSize: 16,
    fontWeight: '600',
    textAlign: 'center',
  },
})

