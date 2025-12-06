"use client"

import { useState } from "react"
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Select } from "@/components/ui/select"
import { Badge } from "@/components/ui/badge"
import { ArrowRight, FlaskConical, Sparkles, RefreshCw, Save } from "lucide-react"
import { motion, AnimatePresence } from "framer-motion"

interface RuleTestDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onSaveRule?: (rule: { table: string; column: string; strategy: string }) => void
}

const STRATEGIES = [
  { value: "email", label: "Email", example: "john.doe@company.com" },
  { value: "phone", label: "Phone", example: "+1 (555) 123-4567" },
  { value: "credit_card", label: "Credit Card", example: "4532-1234-5678-9012" },
  { value: "address", label: "Address", example: "123 Main St, New York, NY" },
  { value: "hash", label: "Hash (SHA-256)", example: "Any sensitive text" },
  { value: "json", label: "JSON Fields", example: '{"ssn": "123-45-6789"}' },
]

// Client-side masking preview functions
const maskEmail = (email: string): string => {
  const parts = email.split("@")
  if (parts.length !== 2) return "***@***.***"
  const name = parts[0]
  const domain = parts[1].split(".")
  const maskedName = name.length > 2 
    ? name[0] + "*".repeat(name.length - 2) + name[name.length - 1]
    : "*".repeat(name.length)
  const maskedDomain = domain.map((d, i) => 
    i === domain.length - 1 ? d : "*".repeat(d.length)
  ).join(".")
  return `${maskedName}@${maskedDomain}`
}

const maskPhone = (phone: string): string => {
  const digits = phone.replace(/\D/g, "")
  if (digits.length < 4) return "***-****"
  return `***-***-${digits.slice(-4)}`
}

const maskCreditCard = (cc: string): string => {
  const digits = cc.replace(/\D/g, "")
  if (digits.length < 4) return "****-****-****-****"
  return `****-****-****-${digits.slice(-4)}`
}

const maskAddress = (address: string): string => {
  const parts = address.split(",")
  if (parts.length === 0) return "*** Masked Address ***"
  return parts.map((p, i) => i === 0 ? "*** " + p.split(" ").pop() : p).join(",")
}

const maskHash = (text: string): string => {
  // Simulate hash with deterministic-looking string
  const hash = Array.from(text).reduce((acc, c) => acc + c.charCodeAt(0), 0)
  return `sha256:${hash.toString(16).padStart(8, "0")}...`
}

const maskJson = (json: string): string => {
  try {
    const obj = JSON.parse(json)
    const masked = Object.fromEntries(
      Object.entries(obj).map(([k, v]) => [k, typeof v === "string" ? "***" : v])
    )
    return JSON.stringify(masked)
  } catch {
    return '{"***": "***"}'
  }
}

const applyMask = (value: string, strategy: string): string => {
  switch (strategy) {
    case "email": return maskEmail(value)
    case "phone": return maskPhone(value)
    case "credit_card": return maskCreditCard(value)
    case "address": return maskAddress(value)
    case "hash": return maskHash(value)
    case "json": return maskJson(value)
    default: return "***"
  }
}

export function RuleTestDialog({ open, onOpenChange, onSaveRule }: RuleTestDialogProps) {
  const [table, setTable] = useState("")
  const [column, setColumn] = useState("")
  const [strategy, setStrategy] = useState("email")
  const [testValue, setTestValue] = useState("")
  const [maskedValue, setMaskedValue] = useState("")
  const [hasTestedRule, setHasTestedRule] = useState(false)

  const selectedStrategy = STRATEGIES.find(s => s.value === strategy)

  const handleTest = () => {
    const masked = applyMask(testValue || selectedStrategy?.example || "", strategy)
    setMaskedValue(masked)
    setHasTestedRule(true)
  }

  const handleSave = () => {
    if (onSaveRule && table && column) {
      onSaveRule({ table, column, strategy })
      onOpenChange(false)
      resetForm()
    }
  }

  const resetForm = () => {
    setTable("")
    setColumn("")
    setStrategy("email")
    setTestValue("")
    setMaskedValue("")
    setHasTestedRule(false)
  }

  const loadExample = () => {
    if (selectedStrategy) {
      setTestValue(selectedStrategy.example)
    }
  }

  return (
    <Dialog open={open} onOpenChange={(isOpen) => {
      if (!isOpen) resetForm()
      onOpenChange(isOpen)
    }}>
      <DialogContent className="sm:max-w-[600px]">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <FlaskConical className="h-5 w-5 text-indigo-400" />
            Test & Create Masking Rule
          </DialogTitle>
          <DialogDescription>
            Test how your data will be masked before applying the rule to your database.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-6 py-4">
          {/* Rule Configuration */}
          <div className="grid grid-cols-2 gap-4">
            <div className="space-y-2">
              <Label htmlFor="table">Table Name</Label>
              <Input
                id="table"
                placeholder="users"
                value={table}
                onChange={(e) => setTable(e.target.value)}
              />
            </div>
            <div className="space-y-2">
              <Label htmlFor="column">Column Name</Label>
              <Input
                id="column"
                placeholder="email"
                value={column}
                onChange={(e) => setColumn(e.target.value)}
              />
            </div>
          </div>

          <div className="space-y-2">
            <Label htmlFor="strategy">Masking Strategy</Label>
            <Select
              id="strategy"
              value={strategy}
              onChange={(e) => {
                setStrategy(e.target.value)
                setHasTestedRule(false)
                setMaskedValue("")
              }}
            >
              {STRATEGIES.map((s) => (
                <option key={s.value} value={s.value}>
                  {s.label}
                </option>
              ))}
            </Select>
          </div>

          {/* Test Area */}
          <div className="bg-gray-800/50 rounded-lg p-4 space-y-4">
            <div className="flex items-center justify-between">
              <h4 className="text-sm font-medium text-gray-300 flex items-center gap-2">
                <Sparkles className="h-4 w-4 text-amber-400" />
                Live Preview
              </h4>
              <Button variant="ghost" size="sm" onClick={loadExample}>
                <RefreshCw className="h-3 w-3 mr-1" />
                Load Example
              </Button>
            </div>

            <div className="space-y-2">
              <Label htmlFor="testValue" className="text-gray-400">
                Test Input
              </Label>
              <Input
                id="testValue"
                placeholder={selectedStrategy?.example || "Enter test value..."}
                value={testValue}
                onChange={(e) => {
                  setTestValue(e.target.value)
                  setHasTestedRule(false)
                }}
              />
            </div>

            <div className="flex items-center justify-center">
              <Button onClick={handleTest} variant="secondary" className="w-full">
                <FlaskConical className="h-4 w-4 mr-2" />
                Test Masking
              </Button>
            </div>

            <AnimatePresence mode="wait">
              {hasTestedRule && (
                <motion.div
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  className="space-y-3"
                >
                  <div className="flex items-center gap-3 text-sm">
                    <div className="flex-1">
                      <span className="text-gray-500 text-xs block mb-1">Original</span>
                      <code className="bg-red-500/10 text-red-400 px-3 py-2 rounded-lg block font-mono text-sm">
                        {testValue || selectedStrategy?.example}
                      </code>
                    </div>
                    <ArrowRight className="h-5 w-5 text-gray-600 flex-shrink-0" />
                    <div className="flex-1">
                      <span className="text-gray-500 text-xs block mb-1">Masked</span>
                      <code className="bg-emerald-500/10 text-emerald-400 px-3 py-2 rounded-lg block font-mono text-sm">
                        {maskedValue}
                      </code>
                    </div>
                  </div>
                  
                  <div className="flex items-center justify-center">
                    <Badge variant="success" className="text-xs">
                      ✓ Masking preview successful
                    </Badge>
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            variant="success"
            onClick={handleSave}
            disabled={!table || !column || !hasTestedRule}
          >
            <Save className="h-4 w-4 mr-2" />
            Save Rule
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
