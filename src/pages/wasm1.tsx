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

export default function WasmSample1() {
  const greetResult = useWasm<string>(greet)

  return (
    <div>
      <span>Hello {greetResult}</span>
    </div>
  )
}
