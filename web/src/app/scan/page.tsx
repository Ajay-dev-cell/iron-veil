"use client"

import { useState } from "react"
import { ScanSearch, ShieldCheck, AlertTriangle, Loader2, CheckCircle } from "lucide-react"

interface Finding {
  table: string
  column: string
  type: string
  confidence: number
  sample: string
}

export default function ScanPage() {
  const [isScanning, setIsScanning] = useState(false)
  const [findings, setFindings] = useState<Finding[]>([])
  const [scanComplete, setScanComplete] = useState(false)
  const [appliedRules, setAppliedRules] = useState<Set<string>>(new Set())

  const startScan = async () => {
    setIsScanning(true)
    setScanComplete(false)
    setFindings([])
    
    try {
      const res = await fetch("http://localhost:3001/scan", { method: "POST" })
      const data = await res.json()
      setFindings(data.findings)
      setScanComplete(true)
    } catch (error) {
      console.error("Scan failed:", error)
    } finally {
      setIsScanning(false)
    }
  }

  const applyRule = async (finding: Finding) => {
    const ruleId = `${finding.table}.${finding.column}`
    
    try {
      await fetch("http://localhost:3001/rules", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          table: finding.table,
          column: finding.column,
          strategy: finding.type === "Email" ? "email" : 
                   finding.type === "Phone" ? "phone" : 
                   finding.type === "CreditCard" ? "credit_card" : "hash"
        })
      })
      
      setAppliedRules(prev => new Set(prev).add(ruleId))
    } catch (error) {
      console.error("Failed to apply rule:", error)
    }
  }

  return (
    <div className="p-8 space-y-8 bg-black min-h-screen text-white">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-3xl font-bold tracking-tight text-white">PII Scanner</h2>
          <p className="text-gray-400 mt-2">
            Scan your database for sensitive information and automatically apply masking rules.
          </p>
        </div>
        <button
          onClick={startScan}
          disabled={isScanning}
          className={`flex items-center px-6 py-3 rounded-lg font-medium transition-colors ${
            isScanning 
              ? "bg-gray-800 text-gray-400 cursor-not-allowed" 
              : "bg-emerald-600 hover:bg-emerald-700 text-white"
          }`}
        >
          {isScanning ? (
            <>
              <Loader2 className="w-5 h-5 mr-2 animate-spin" />
              Scanning Database...
            </>
          ) : (
            <>
              <ScanSearch className="w-5 h-5 mr-2" />
              Start New Scan
            </>
          )}
        </button>
      </div>

      {/* Results Area */}
      <div className="space-y-4">
        {findings.length > 0 && (
          <div className="grid gap-4">
            {findings.map((finding, idx) => {
              const ruleId = `${finding.table}.${finding.column}`
              const isApplied = appliedRules.has(ruleId)

              return (
                <div 
                  key={idx}
                  className="bg-gray-900 border border-gray-800 rounded-xl p-6 flex items-center justify-between hover:border-gray-700 transition-colors"
                >
                  <div className="flex items-start space-x-4">
                    <div className="p-3 bg-red-500/10 rounded-lg">
                      <AlertTriangle className="w-6 h-6 text-red-500" />
                    </div>
                    <div>
                      <div className="flex items-center space-x-2">
                        <h3 className="text-lg font-semibold text-white">
                          {finding.table}.{finding.column}
                        </h3>
                        <span className="px-2 py-1 text-xs font-medium bg-red-500/20 text-red-400 rounded-full border border-red-500/20">
                          {finding.type}
                        </span>
                        <span className="px-2 py-1 text-xs font-medium bg-gray-800 text-gray-400 rounded-full">
                          {(finding.confidence * 100).toFixed(0)}% Confidence
                        </span>
                      </div>
                      <p className="text-gray-400 mt-1 text-sm">
                        Sample detected: <code className="bg-gray-950 px-1 py-0.5 rounded text-gray-300">{finding.sample}</code>
                      </p>
                    </div>
                  </div>

                  <button
                    onClick={() => applyRule(finding)}
                    disabled={isApplied}
                    className={`flex items-center px-4 py-2 rounded-lg text-sm font-medium transition-colors ${
                      isApplied
                        ? "bg-emerald-500/10 text-emerald-500 border border-emerald-500/20 cursor-default"
                        : "bg-white text-black hover:bg-gray-200"
                    }`}
                  >
                    {isApplied ? (
                      <>
                        <CheckCircle className="w-4 h-4 mr-2" />
                        Rule Applied
                      </>
                    ) : (
                      <>
                        <ShieldCheck className="w-4 h-4 mr-2" />
                        Apply Masking
                      </>
                    )}
                  </button>
                </div>
              )
            })}
          </div>
        )}

        {scanComplete && findings.length === 0 && (
          <div className="text-center py-20 bg-gray-900/50 rounded-xl border border-gray-800 border-dashed">
            <ShieldCheck className="w-12 h-12 text-emerald-500 mx-auto mb-4" />
            <h3 className="text-xl font-semibold text-white">No PII Detected</h3>
            <p className="text-gray-400 mt-2">Your database appears to be clean based on the current scan rules.</p>
          </div>
        )}

        {!isScanning && !scanComplete && findings.length === 0 && (
          <div className="text-center py-20 bg-gray-900/50 rounded-xl border border-gray-800 border-dashed">
            <ScanSearch className="w-12 h-12 text-gray-600 mx-auto mb-4" />
            <h3 className="text-xl font-semibold text-gray-400">Ready to Scan</h3>
            <p className="text-gray-500 mt-2">Click the button above to analyze your database for sensitive data.</p>
          </div>
        )}
      </div>
    </div>
  )
}
