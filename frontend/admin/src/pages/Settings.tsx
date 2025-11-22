export default function Settings() {
  return (
    <div>
      <h1 className="text-3xl font-bold text-gray-900 mb-6">Settings</h1>
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 mb-4">Identity Providers</h2>
        <p className="text-gray-500 mb-6">Configure identity provider integrations.</p>
        <button className="bg-primary-600 text-white px-4 py-2 rounded-lg hover:bg-primary-700">
          Add Identity Provider
        </button>
      </div>
    </div>
  )
}

