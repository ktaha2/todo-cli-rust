const reportWebVitals = (onPerfEntry?: Function) => {
  if (onPerfEntry && onPerfEntry instanceof Function) {
    import('web-vitals').then((webVitals) => {
      // Handle different versions of web-vitals
      const { getCLS, getFID, getFCP, getLCP, getTTFB } = webVitals as any;
      if (getCLS) getCLS(onPerfEntry);
      if (getFID) getFID(onPerfEntry);
      if (getFCP) getFCP(onPerfEntry);
      if (getLCP) getLCP(onPerfEntry);
      if (getTTFB) getTTFB(onPerfEntry);
    }).catch(() => {
      // Ignore web vitals errors in development
    });
  }
};

export default reportWebVitals;
