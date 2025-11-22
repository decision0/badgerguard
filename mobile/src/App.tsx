import React from 'react'
import { NavigationContainer } from '@react-navigation/native'
import { createStackNavigator } from '@react-navigation/stack'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import HomeScreen from './screens/HomeScreen'
import PushChallengeScreen from './screens/PushChallengeScreen'
import TotpScreen from './screens/TotpScreen'
import SettingsScreen from './screens/SettingsScreen'

const Stack = createStackNavigator()
const queryClient = new QueryClient()

export default function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <NavigationContainer>
        <Stack.Navigator initialRouteName="Home">
          <Stack.Screen name="Home" component={HomeScreen} />
          <Stack.Screen name="PushChallenge" component={PushChallengeScreen} />
          <Stack.Screen name="Totp" component={TotpScreen} />
          <Stack.Screen name="Settings" component={SettingsScreen} />
        </Stack.Navigator>
      </NavigationContainer>
    </QueryClientProvider>
  )
}

