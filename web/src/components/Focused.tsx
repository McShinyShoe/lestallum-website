import React, { ReactNode } from 'react'

export default function Focused({ children }: { children: ReactNode }) {
  return (
    <div className="max-w-250 my-5 mx-auto">
        {children}
    </div>
  )
}