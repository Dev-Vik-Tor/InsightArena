"use client";

import { Bell, TrendingUp, Trophy, Gift, Users, Clock } from "lucide-react";

interface NotificationItem {
  id: string;
  type: "market" | "competition" | "reward" | "social";
  icon: "trend" | "trophy" | "gift" | "users";
  message: string;
  timestamp: string;
  isRead: boolean;
}

// Mock data for demonstration
const mockNotifications: NotificationItem[] = [
  {
    id: "1",
    type: "market",
    icon: "trend",
    message: "Your prediction on 'Bitcoin Price by EOY' is currently winning!",
    timestamp: "2h ago",
    isRead: false,
  },
  {
    id: "2",
    type: "competition",
    icon: "trophy",
    message: "You've moved up to 3rd place in the Weekly Leaderboard",
    timestamp: "4h ago",
    isRead: false,
  },
  {
    id: "3",
    type: "reward",
    icon: "gift",
    message: "Congratulations! You've earned 50 XLM from your winning prediction",
    timestamp: "1d ago",
    isRead: true,
  },
  {
    id: "4",
    type: "social",
    icon: "users",
    message: "Alex invited you to join the 'Crypto Predictions' private competition",
    timestamp: "2d ago",
    isRead: true,
  },
  {
    id: "5",
    type: "market",
    icon: "trend",
    message: "Market 'US Election Results' closes in 24 hours",
    timestamp: "3d ago",
    isRead: true,
  },
];

const iconMap = {
  trend: TrendingUp,
  trophy: Trophy,
  gift: Gift,
  users: Users,
};

const iconColorMap = {
  trend: "text-blue-400",
  trophy: "text-yellow-400",
  gift: "text-green-400",
  users: "text-purple-400",
};

const iconBgMap = {
  trend: "bg-blue-400/10",
  trophy: "bg-yellow-400/10",
  gift: "bg-green-400/10",
  users: "bg-purple-400/10",
};

export default function NotificationsCard() {
  const unreadCount = mockNotifications.filter(n => !n.isRead).length;

  return (
    <div className="relative bg-[#0f172a] rounded-2xl p-5 w-full shadow-lg overflow-hidden">
      {/* TOP GOLD EDGE (same gold as UI) */}
      <div className="absolute top-0 left-0 right-0 h-[3px] bg-[#F5C451]/70 rounded-t-2xl" />

      {/* Header */}
      <div className="flex items-center justify-between mb-5">
        <div className="flex items-center gap-3">
          <Bell className="h-5 w-5 text-[#F5C451]" />
          <h2 className="text-white font-semibold">Notifications</h2>
        </div>
        {unreadCount > 0 && (
          <div className="relative">
            <div className="w-2 h-2 bg-red-500 rounded-full animate-pulse" />
          </div>
        )}
      </div>

      {/* Notifications List */}
      <div className="space-y-0">
        {mockNotifications.map((notification, index) => {
          const IconComponent = iconMap[notification.icon];
          const isLast = index === mockNotifications.length - 1;
          
          return (
            <div key={notification.id}>
              <div className={`flex items-start gap-3 py-3 ${!notification.isRead ? 'opacity-100' : 'opacity-70'}`}>
                {/* Icon Box */}
                <div className={`flex-shrink-0 w-8 h-8 rounded-lg ${iconBgMap[notification.icon]} flex items-center justify-center`}>
                  <IconComponent className={`h-4 w-4 ${iconColorMap[notification.icon]}`} />
                </div>
                
                {/* Content */}
                <div className="flex-1 min-w-0">
                  <p className="text-sm text-gray-300 leading-relaxed">
                    {notification.message}
                  </p>
                  <div className="flex items-center gap-1 mt-1">
                    <Clock className="h-3 w-3 text-gray-500" />
                    <span className="text-xs text-gray-500">
                      {notification.timestamp}
                    </span>
                  </div>
                </div>
                
                {/* Unread indicator */}
                {!notification.isRead && (
                  <div className="flex-shrink-0 w-2 h-2 bg-[#4FD1C5] rounded-full mt-2" />
                )}
              </div>
              
              {/* Divider line (subtle, matching Figma stroke color) */}
              {!isLast && (
                <div className="border-b border-gray-700/30" />
              )}
            </div>
          );
        })}
      </div>

      {/* View All Button */}
      <button className="mt-4 w-full py-2 rounded-lg border border-gray-600/30 text-gray-300 text-sm font-medium hover:bg-white/5 transition">
        View All Notifications
      </button>
    </div>
  );
}