// Environment configuration
const getConfig = () => {
  const apiBaseUrl = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8080';
  
  return {
    apiBaseUrl,
  };
};

export const config = getConfig();