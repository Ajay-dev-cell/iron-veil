"use client"

import { ReactNode } from "react"
import { cn } from "@/lib/utils"
import { TrendingUp, TrendingDown, Minus } from "lucide-react"

interface StatsCardProps {
  title: string
  value: string | number
  description?: string
  icon?: ReactNode
  trend?: {
    value: number
    label: string
  }
  className?: string
  variant?: "default" | "success" | "warning" | "danger"
}

export function StatsCard({
  title,
  value,
  description,
  icon,
  trend,
  className,
  variant = "default",
}: StatsCardProps) {
  const variantStyles = {
    default: "bg-gray-900 border-gray-800",
    success: "bg-emerald-500/5 border-emerald-500/20",
    warning: "bg-amber-500/5 border-amber-500/20",
    danger: "bg-red-500/5 border-red-500/20",
  }

  const iconColors = {
    default: "text-indigo-400 bg-indigo-500/10",
    success: "text-emerald-400 bg-emerald-500/10",
    warning: "text-amber-400 bg-amber-500/10",
    danger: "text-red-400 bg-red-500/10",
  }

  const getTrendIcon = () => {
    if (!trend) return null
    if (trend.value > 0) return <TrendingUp className="h-3 w-3" />
    if (trend.value < 0) return <TrendingDown className="h-3 w-3" />
    return <Minus className="h-3 w-3" />
  }

  const getTrendColor = () => {
    if (!trend) return ""
    if (trend.value > 0) return "text-emerald-400"
    if (trend.value < 0) return "text-red-400"
    return "text-gray-400"
  }

  return (
    <div
      className={cn(
        "rounded-xl border p-6 transition-all hover:shadow-lg h-[140px]",
        variantStyles[variant],
        className
      )}
    >
      <div className="flex items-start justify-between h-full">
        <div className="space-y-2 flex flex-col justify-between h-full">
          <div>
            <p className="text-sm font-medium text-gray-400">{title}</p>
            <div className="flex items-baseline gap-2 mt-1">
              <p className="text-3xl font-bold text-white">{value}</p>
              {trend && (
                <span className={cn("flex items-center gap-1 text-xs font-medium", getTrendColor())}>
                  {getTrendIcon()}
                  {Math.abs(trend.value)}%
                </span>
              )}
            </div>
          </div>
          <div className="min-h-[2rem]">
            {description && (
              <p className="text-xs text-gray-500">{description}</p>
            )}
            {trend && (
              <p className="text-xs text-gray-500">{trend.label}</p>
            )}
          </div>
        </div>
        {icon && (
          <div className={cn("p-3 rounded-lg flex-shrink-0", iconColors[variant])}>
            {icon}
          </div>
        )}
      </div>
    </div>
  )
}
