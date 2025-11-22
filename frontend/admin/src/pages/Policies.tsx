export default function Policies() {
  return (
    <div>
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold text-gray-900">Policies</h1>
        <button className="bg-primary-600 text-white px-4 py-2 rounded-lg hover:bg-primary-700">
          New Policy
        </button>
      </div>
      <div className="bg-white rounded-lg shadow">
        <p className="p-8 text-gray-500">No policies configured yet.</p>
      </div>
    </div>
  )
}

