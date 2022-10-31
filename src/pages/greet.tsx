import { useCallback, useState } from 'react'
import { greet } from '../../calculator/pkg'
import { wasmFn } from '@/utility/wasm'

export default function Greet() {
  const [result, setResult] = useState('')

  const handleClick = useCallback(async () => {
    setResult(await wasmFn(greet))
  }, [])
  return (
    <main className="min-h-screen w-screen flex justify-center items-center flex-col bg-amber-900">
      <button onClick={handleClick}>click</button>
      <div className="text-amber-50">Hello {result}</div>
    </main>
  )
}
