import React from 'react';

type PillProps = {
  children: React.ReactNode;
  className?: string;
};

const Pill: React.FC<PillProps> = ({ children, className = '' }) => {
  return (
    <div className={`bg-gray-100 rounded-lg gap-2 flex px-2 py-1 justify-center align-middle ${className}`}>
      {children}
    </div>
  );
};

export default Pill;