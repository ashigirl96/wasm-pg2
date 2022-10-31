import { useEffect, useState } from 'react'
import wasmModule, { greet } from '@/../calculator/pkg'

function useWasm<T>(fn: () => T) {
  const [result, setResult] = useState<T>()
  useEffect(() => {
    ;(async () => {
      setResult(await wasmModule().then(() => fn()))
    })()
  }, [fn, setResult])
  return result
}

export default function WasmSample2() {
  return (
    <main className="h-screen w-screen bg-amber-900">
      <div className="flex justify-center items-center">
        <span>Hello</span>
      </div>
    </main>
  )
}
