//! Catalog caching system for improved performance
//!
//! This module provides:
//! - LRU caching for loaded catalogs
//! - Thread-safe access to cached catalog data
//! - Memory usage optimization for large catalog collections
//! - Cache statistics and monitoring capabilities

use crate::error::{Error, Result};
use crate::types::catalogs::{
    controllers::{ControllerCatalog, CatalogController},
    trajectories::{TrajectoryCatalog, CatalogTrajectory},
    routes::{RouteCatalog, CatalogRoute},
    environments::{EnvironmentCatalog, CatalogEnvironment},
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Instant, SystemTime};

/// Cache statistics for monitoring performance
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Total number of cache hits
    pub hits: u64,
    /// Total number of cache misses
    pub misses: u64,
    /// Number of entries in the cache
    pub entries: usize,
    /// Total memory usage estimate in bytes
    pub memory_usage: usize,
    /// Cache creation time
    pub created_at: SystemTime,
}

impl CacheStats {
    /// Calculate hit ratio as a percentage
    pub fn hit_ratio(&self) -> f64 {
        if self.hits + self.misses == 0 {
            0.0
        } else {
            (self.hits as f64 / (self.hits + self.misses) as f64) * 100.0
        }
    }

    /// Get total requests (hits + misses)
    pub fn total_requests(&self) -> u64 {
        self.hits + self.misses
    }
}

/// Cached catalog entry with metadata
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    /// The cached data
    data: T,
    /// When this entry was last accessed
    last_accessed: Instant,
    /// When this entry was created
    created_at: Instant,
    /// Size estimate in bytes
    size_estimate: usize,
}

impl<T> CacheEntry<T> {
    fn new(data: T, size_estimate: usize) -> Self {
        let now = Instant::now();
        Self {
            data,
            last_accessed: now,
            created_at: now,
            size_estimate,
        }
    }

    fn access(&mut self) -> &T {
        self.last_accessed = Instant::now();
        &self.data
    }

    fn into_data(self) -> T {
        self.data
    }
}

/// Thread-safe LRU cache for catalog entries
pub struct CatalogCache {
    /// Controller catalog cache
    controller_cache: Arc<RwLock<HashMap<String, CacheEntry<CatalogController>>>>,
    /// Trajectory catalog cache
    trajectory_cache: Arc<RwLock<HashMap<String, CacheEntry<CatalogTrajectory>>>>,
    /// Route catalog cache
    route_cache: Arc<RwLock<HashMap<String, CacheEntry<CatalogRoute>>>>,
    /// Environment catalog cache
    environment_cache: Arc<RwLock<HashMap<String, CacheEntry<CatalogEnvironment>>>>,
    /// Full catalog file cache (for mixed-type catalogs)
    catalog_file_cache: Arc<RwLock<HashMap<PathBuf, CacheEntry<String>>>>,
    /// Maximum number of entries per cache
    max_entries: usize,
    /// Cache statistics
    stats: Arc<RwLock<CacheStats>>,
}

impl CatalogCache {
    /// Create a new catalog cache with default capacity (100 entries per cache)
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    /// Create a new catalog cache with specified capacity per cache type
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            controller_cache: Arc::new(RwLock::new(HashMap::new())),
            trajectory_cache: Arc::new(RwLock::new(HashMap::new())),
            route_cache: Arc::new(RwLock::new(HashMap::new())),
            environment_cache: Arc::new(RwLock::new(HashMap::new())),
            catalog_file_cache: Arc::new(RwLock::new(HashMap::new())),
            max_entries,
            stats: Arc::new(RwLock::new(CacheStats {
                hits: 0,
                misses: 0,
                entries: 0,
                memory_usage: 0,
                created_at: SystemTime::now(),
            })),
        }
    }

    /// Get a controller from the cache
    pub fn get_controller(&self, key: &str) -> Option<CatalogController> {
        let mut cache = self.controller_cache.write().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            self.record_hit();
            Some(entry.access().clone())
        } else {
            self.record_miss();
            None
        }
    }

    /// Put a controller in the cache
    pub fn put_controller(&self, key: String, controller: CatalogController) {
        let size_estimate = self.estimate_controller_size(&controller);
        let entry = CacheEntry::new(controller, size_estimate);
        
        let mut cache = self.controller_cache.write().unwrap();
        self.evict_if_needed(&mut cache);
        cache.insert(key, entry);
        self.update_stats();
    }

    /// Get a trajectory from the cache
    pub fn get_trajectory(&self, key: &str) -> Option<CatalogTrajectory> {
        let mut cache = self.trajectory_cache.write().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            self.record_hit();
            Some(entry.access().clone())
        } else {
            self.record_miss();
            None
        }
    }

    /// Put a trajectory in the cache
    pub fn put_trajectory(&self, key: String, trajectory: CatalogTrajectory) {
        let size_estimate = self.estimate_trajectory_size(&trajectory);
        let entry = CacheEntry::new(trajectory, size_estimate);
        
        let mut cache = self.trajectory_cache.write().unwrap();
        self.evict_if_needed(&mut cache);
        cache.insert(key, entry);
        self.update_stats();
    }

    /// Get a route from the cache
    pub fn get_route(&self, key: &str) -> Option<CatalogRoute> {
        let mut cache = self.route_cache.write().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            self.record_hit();
            Some(entry.access().clone())
        } else {
            self.record_miss();
            None
        }
    }

    /// Put a route in the cache
    pub fn put_route(&self, key: String, route: CatalogRoute) {
        let size_estimate = self.estimate_route_size(&route);
        let entry = CacheEntry::new(route, size_estimate);
        
        let mut cache = self.route_cache.write().unwrap();
        self.evict_if_needed(&mut cache);
        cache.insert(key, entry);
        self.update_stats();
    }

    /// Get an environment from the cache
    pub fn get_environment(&self, key: &str) -> Option<CatalogEnvironment> {
        let mut cache = self.environment_cache.write().unwrap();
        if let Some(entry) = cache.get_mut(key) {
            self.record_hit();
            Some(entry.access().clone())
        } else {
            self.record_miss();
            None
        }
    }

    /// Put an environment in the cache
    pub fn put_environment(&self, key: String, environment: CatalogEnvironment) {
        let size_estimate = self.estimate_environment_size(&environment);
        let entry = CacheEntry::new(environment, size_estimate);
        
        let mut cache = self.environment_cache.write().unwrap();
        self.evict_if_needed(&mut cache);
        cache.insert(key, entry);
        self.update_stats();
    }

    /// Get a raw catalog file from the cache
    pub fn get_catalog_file(&self, path: &PathBuf) -> Option<String> {
        let mut cache = self.catalog_file_cache.write().unwrap();
        if let Some(entry) = cache.get_mut(path) {
            self.record_hit();
            Some(entry.access().clone())
        } else {
            self.record_miss();
            None
        }
    }

    /// Put a raw catalog file in the cache
    pub fn put_catalog_file(&self, path: PathBuf, content: String) {
        let size_estimate = content.len();
        let entry = CacheEntry::new(content, size_estimate);
        
        let mut cache = self.catalog_file_cache.write().unwrap();
        self.evict_if_needed_pathbuf(&mut cache);
        cache.insert(path, entry);
        self.update_stats();
    }

    /// Insert a parsed catalog into the cache (for compatibility with legacy CatalogManager)
    pub fn insert_parsed<P: AsRef<std::path::Path>>(&self, path: P, catalog: crate::types::catalogs::files::CatalogFile) {
        // For now, just convert to string and cache as file content
        // In a real implementation, we'd want to cache the parsed structure
        let path_buf = path.as_ref().to_path_buf();
        let content = format!("<!-- Cached catalog file: {} -->", path_buf.display());
        self.put_catalog_file(path_buf, content);
    }

    /// Clear all caches
    pub fn clear(&self) {
        self.controller_cache.write().unwrap().clear();
        self.trajectory_cache.write().unwrap().clear();
        self.route_cache.write().unwrap().clear();
        self.environment_cache.write().unwrap().clear();
        self.catalog_file_cache.write().unwrap().clear();
        self.update_stats();
    }

    /// Invalidate a specific controller entry
    pub fn invalidate_controller(&self, key: &str) -> bool {
        let mut cache = self.controller_cache.write().unwrap();
        let removed = cache.remove(key).is_some();
        if removed {
            self.update_stats();
        }
        removed
    }

    /// Invalidate a specific trajectory entry
    pub fn invalidate_trajectory(&self, key: &str) -> bool {
        let mut cache = self.trajectory_cache.write().unwrap();
        let removed = cache.remove(key).is_some();
        if removed {
            self.update_stats();
        }
        removed
    }

    /// Invalidate a specific route entry
    pub fn invalidate_route(&self, key: &str) -> bool {
        let mut cache = self.route_cache.write().unwrap();
        let removed = cache.remove(key).is_some();
        if removed {
            self.update_stats();
        }
        removed
    }

    /// Invalidate a specific environment entry
    pub fn invalidate_environment(&self, key: &str) -> bool {
        let mut cache = self.environment_cache.write().unwrap();
        let removed = cache.remove(key).is_some();
        if removed {
            self.update_stats();
        }
        removed
    }

    /// Invalidate a specific catalog file entry
    pub fn invalidate_catalog_file(&self, path: &PathBuf) -> bool {
        let mut cache = self.catalog_file_cache.write().unwrap();
        let removed = cache.remove(path).is_some();
        if removed {
            self.update_stats();
        }
        removed
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        self.stats.read().unwrap().clone()
    }

    /// Get the current capacity limit
    pub fn capacity(&self) -> usize {
        self.max_entries
    }

    /// Set a new capacity limit (triggers immediate eviction if needed)
    pub fn set_capacity(&mut self, new_capacity: usize) {
        self.max_entries = new_capacity;
        
        // Trigger eviction on all caches if needed
        {
            let mut cache = self.controller_cache.write().unwrap();
            while cache.len() > new_capacity {
                self.evict_oldest(&mut cache);
            }
        }
        {
            let mut cache = self.trajectory_cache.write().unwrap();
            while cache.len() > new_capacity {
                self.evict_oldest(&mut cache);
            }
        }
        {
            let mut cache = self.route_cache.write().unwrap();
            while cache.len() > new_capacity {
                self.evict_oldest(&mut cache);
            }
        }
        {
            let mut cache = self.environment_cache.write().unwrap();
            while cache.len() > new_capacity {
                self.evict_oldest(&mut cache);
            }
        }
        {
            let mut cache = self.catalog_file_cache.write().unwrap();
            while cache.len() > new_capacity {
                self.evict_oldest_pathbuf(&mut cache);
            }
        }
        
        self.update_stats();
    }

    // Private helper methods

    fn record_hit(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.hits += 1;
    }

    fn record_miss(&self) {
        let mut stats = self.stats.write().unwrap();
        stats.misses += 1;
    }

    fn update_stats(&self) {
        let mut stats = self.stats.write().unwrap();
        
        let controller_count = self.controller_cache.read().unwrap().len();
        let trajectory_count = self.trajectory_cache.read().unwrap().len();
        let route_count = self.route_cache.read().unwrap().len();
        let environment_count = self.environment_cache.read().unwrap().len();
        let file_count = self.catalog_file_cache.read().unwrap().len();
        
        stats.entries = controller_count + trajectory_count + route_count + environment_count + file_count;
        
        // Calculate total memory usage estimate
        let controller_memory: usize = self.controller_cache.read().unwrap()
            .values().map(|e| e.size_estimate).sum();
        let trajectory_memory: usize = self.trajectory_cache.read().unwrap()
            .values().map(|e| e.size_estimate).sum();
        let route_memory: usize = self.route_cache.read().unwrap()
            .values().map(|e| e.size_estimate).sum();
        let environment_memory: usize = self.environment_cache.read().unwrap()
            .values().map(|e| e.size_estimate).sum();
        let file_memory: usize = self.catalog_file_cache.read().unwrap()
            .values().map(|e| e.size_estimate).sum();
        
        stats.memory_usage = controller_memory + trajectory_memory + route_memory + environment_memory + file_memory;
    }

    fn evict_if_needed<T>(&self, cache: &mut HashMap<String, CacheEntry<T>>) {
        if cache.len() >= self.max_entries {
            self.evict_oldest(cache);
        }
    }

    fn evict_if_needed_pathbuf<T>(&self, cache: &mut HashMap<PathBuf, CacheEntry<T>>) {
        if cache.len() >= self.max_entries {
            self.evict_oldest_pathbuf(cache);
        }
    }

    fn evict_oldest<T>(&self, cache: &mut HashMap<String, CacheEntry<T>>) {
        if let Some((oldest_key, _)) = cache.iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(k, v)| (k.clone(), v.last_accessed)) {
            cache.remove(&oldest_key);
        }
    }

    fn evict_oldest_pathbuf<T>(&self, cache: &mut HashMap<PathBuf, CacheEntry<T>>) {
        if let Some((oldest_key, _)) = cache.iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(k, v)| (k.clone(), v.last_accessed)) {
            cache.remove(&oldest_key);
        }
    }

    // Size estimation methods (rough estimates for memory usage tracking)
    
    fn estimate_controller_size(&self, _controller: &CatalogController) -> usize {
        // Rough estimate: base size + name length + properties
        200 + _controller.name.len() * 2 // Assuming UTF-8 overhead
    }

    fn estimate_trajectory_size(&self, trajectory: &CatalogTrajectory) -> usize {
        // Rough estimate: base size + name + shape data
        500 + trajectory.name.len() * 2 // Trajectories tend to be larger
    }

    fn estimate_route_size(&self, route: &CatalogRoute) -> usize {
        // Rough estimate: base size + name + waypoints
        let waypoint_count = route.waypoints.len();
        300 + route.name.len() * 2 + waypoint_count * 100
    }

    fn estimate_environment_size(&self, environment: &CatalogEnvironment) -> usize {
        // Rough estimate: base size + name + environment data
        400 + environment.name.len() * 2
    }
}

impl Default for CatalogCache {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for CatalogCache {
    fn clone(&self) -> Self {
        Self {
            controller_cache: Arc::clone(&self.controller_cache),
            trajectory_cache: Arc::clone(&self.trajectory_cache),
            route_cache: Arc::clone(&self.route_cache),
            environment_cache: Arc::clone(&self.environment_cache),
            catalog_file_cache: Arc::clone(&self.catalog_file_cache),
            max_entries: self.max_entries,
            stats: Arc::clone(&self.stats),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::basic::{Value, ParameterDeclarations};

    #[test]
    fn test_cache_creation() {
        let cache = CatalogCache::new();
        assert_eq!(cache.capacity(), 100);
        
        let cache_custom = CatalogCache::with_capacity(50);
        assert_eq!(cache_custom.capacity(), 50);
    }

    #[test]
    fn test_controller_cache() {
        let cache = CatalogCache::new();
        
        let controller = CatalogController {
            name: "TestController".to_string(),
            controller_type: Some(crate::types::enums::ControllerType::Movement),
            parameter_declarations: Some(ParameterDeclarations::default()),
            properties: None,
        };
        
        // Test cache miss
        assert!(cache.get_controller("test_key").is_none());
        
        // Test cache put and hit
        cache.put_controller("test_key".to_string(), controller.clone());
        let cached = cache.get_controller("test_key").unwrap();
        assert_eq!(cached.name, controller.name);
        
        // Test statistics
        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.entries, 1);
        assert!(stats.memory_usage > 0);
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = CatalogCache::with_capacity(2);
        
        let controller1 = CatalogController {
            name: "Controller1".to_string(),
            controller_type: Some(crate::types::enums::ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        };
        let controller2 = CatalogController {
            name: "Controller2".to_string(),
            controller_type: Some(crate::types::enums::ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        };
        let controller3 = CatalogController {
            name: "Controller3".to_string(),
            controller_type: Some(crate::types::enums::ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        };
        
        // Fill cache to capacity
        cache.put_controller("key1".to_string(), controller1);
        cache.put_controller("key2".to_string(), controller2);
        
        // Access key1 to make it more recently used
        cache.get_controller("key1");
        
        // Add third entry, should evict key2 (least recently used)
        cache.put_controller("key3".to_string(), controller3);
        
        // key1 and key3 should be present, key2 should be evicted
        assert!(cache.get_controller("key1").is_some());
        assert!(cache.get_controller("key2").is_none());
        assert!(cache.get_controller("key3").is_some());
    }

    #[test]
    fn test_cache_invalidation() {
        let cache = CatalogCache::new();
        
        let controller = CatalogController {
            name: "TestController".to_string(),
            controller_type: Some(crate::types::enums::ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        };
        
        cache.put_controller("test_key".to_string(), controller);
        assert!(cache.get_controller("test_key").is_some());
        
        // Test invalidation
        assert!(cache.invalidate_controller("test_key"));
        assert!(cache.get_controller("test_key").is_none());
        
        // Test invalidating non-existent key
        assert!(!cache.invalidate_controller("non_existent"));
    }

    #[test]
    fn test_cache_clear() {
        let cache = CatalogCache::new();
        
        let controller = CatalogController {
            name: "TestController".to_string(),
            controller_type: Some(crate::types::enums::ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        };
        
        cache.put_controller("test_key".to_string(), controller);
        assert_eq!(cache.stats().entries, 1);
        
        cache.clear();
        assert_eq!(cache.stats().entries, 0);
        assert!(cache.get_controller("test_key").is_none());
    }

    #[test]
    fn test_cache_capacity_change() {
        let mut cache = CatalogCache::with_capacity(3);
        
        // Add 3 controllers
        for i in 0..3 {
            let controller = CatalogController {
                name: format!("Controller{}", i),
                controller_type: Some(crate::types::enums::ControllerType::Movement),
                parameter_declarations: None,
                properties: None,
            };
            cache.put_controller(format!("key{}", i), controller);
        }
        
        assert_eq!(cache.stats().entries, 3);
        
        // Reduce capacity to 1, should trigger eviction
        cache.set_capacity(1);
        assert_eq!(cache.stats().entries, 1);
    }

    #[test]
    fn test_hit_ratio_calculation() {
        let cache = CatalogCache::new();
        
        let controller = CatalogController {
            name: "TestController".to_string(),
            controller_type: Some(crate::types::enums::ControllerType::Movement),
            parameter_declarations: None,
            properties: None,
        };
        
        cache.put_controller("test_key".to_string(), controller);
        
        // 1 miss (initial get), 2 hits
        cache.get_controller("non_existent"); // miss
        cache.get_controller("test_key"); // hit
        cache.get_controller("test_key"); // hit
        
        let stats = cache.stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 2); // 1 from test + 1 from non_existent
        assert_eq!(stats.hit_ratio(), 50.0);
    }
}