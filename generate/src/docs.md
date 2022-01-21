<!-- file * -->
<!-- const INTEREST_MATCH_ALL -->
<!-- const ITERATOR_METHODS_VERSION -->
<!-- const LOG_LEVEL_TRACE -->
<!-- const OBJECT_FEATURES_ALL -->
Special value that can be used to activate all the supported features in any given object.
<!-- static OBJECT_FORMAT -->
<!-- const PIPEWIRE_OBJECT_FEATURES_ALL -->
<!-- const PIPEWIRE_OBJECT_FEATURES_MINIMAL -->
<!-- const SPA_TYPE_INVALID -->
Type id representing an invalid SPA type.
<!-- struct Client -->


# Implements

[`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- impl Client::fn send_error -->
Send an error to the client.
## `id`
the global id to report the error on
## `res`
an errno style error code
## `message`
the error message string
<!-- impl Client::fn update_permissions -->
Update client's permissions on a list of objects.


An object id of -1 can be used to set the default object permissions for this client
## `n_perm`
the number of permissions specified in the variable arguments
<!-- impl Client::fn update_permissions_array -->
Update client's permissions on a list of objects.


An object id of -1 can be used to set the default object permissions for this client
## `permissions`
an array of permissions per object id
<!-- struct ComponentLoader -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`PluginExt`][trait@crate::prelude::PluginExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- enum ConstraintType -->
Constraint types for [`ObjectInterest::add_constraint()`][crate::ObjectInterest::add_constraint()]
<!-- enum ConstraintVerb -->
Verbs to use with [`ObjectInterest::add_constraint()`][crate::ObjectInterest::add_constraint()]
<!-- struct Core -->


# Implements

[`trait@glib::ObjectExt`]
<!-- impl Core::fn new -->
Creates a new core object.
## `context`
the GMainContext to use for events
## `properties`
additional properties, which are passed to `pw_context_new()` and `pw_context_connect()`

# Returns

a new WpCore
<!-- impl Core::fn connect -->
Connects this core to the PipeWire server.


When connection succeeds, the WpCore "connected" signal is emitted.

# Returns

TRUE if the core is effectively connected or FALSE if connection failed
<!-- impl Core::fn disconnect -->
Disconnects this core from the PipeWire server.


This also effectively destroys all WpCore objects that were created through the registry, destroys the pw_core and finally emits the WpCore "disconnected" signal.
<!-- impl Core::fn g_main_context -->
Gets the GMainContext of the core.

# Returns

the GMainContext that is in use by this core for events
<!-- impl Core::fn properties -->
Gets the properties of the core.

# Returns

the properties of `self`
<!-- impl Core::fn pw_context -->
Gets the internal PipeWire context of the core.

# Returns

the internal pw_context object
<!-- impl Core::fn pw_core -->
Gets the internal PipeWire core of the core.

# Returns

the internal pw_core object, or NULL if the core is not connected to PipeWire
<!-- impl Core::fn remote_cookie -->
Gets the cookie of the core's connected PipeWire instance.

# Returns

The cookie of the PipeWire instance that `self` is connected to. The cookie is a unique random number for identifying an instance of PipeWire
<!-- impl Core::fn remote_host_name -->
Gets the host name of the core's connected PipeWire instance.

# Returns

The name of the host where the PipeWire instance that `self` is connected to is running on
<!-- impl Core::fn remote_name -->
Gets the name of the core's connected PipeWire instance.

# Returns

The name of the PipeWire instance that `self` is connected to
<!-- impl Core::fn remote_properties -->
Gets the properties of the core's connected PipeWire instance.

# Returns

the properties of the PipeWire instance that `self` is connected to
<!-- impl Core::fn remote_user_name -->
Gets the user name of the core's connected PipeWire instance.

# Returns

The name of the user that started the PipeWire instance that `self` is connected to
<!-- impl Core::fn remote_version -->
Gets the version of the core's connected PipeWire instance.

# Returns

The version of the PipeWire instance that `self` is connected to
<!-- impl Core::fn idle_add -->
Adds an idle callback to be called in the same GMainContext as the one used by this core.


This is essentially the same as `g_idle_add_full()`, but it adds the created GSource on the GMainContext used by this core instead of the default context.
## `function`
the function to call

# Returns


## `source`
the source
<!-- impl Core::fn idle_add_closure -->
Adds an idle callback to be called in the same GMainContext as the one used by this core.


This is the same as [`idle_add()`][Self::idle_add()], but it allows you to specify a GClosure instead of a C callback.
## `closure`
the closure to invoke

# Returns


## `source`
the source
<!-- impl Core::fn install_object_manager -->
Installs the object manager on this core, activating its internal management engine.


This will immediately emit signals about objects added on `om` if objects that the `om` is interested in were in existence already.
## `om`
a WpObjectManager
<!-- impl Core::fn is_connected -->
Checks if the core is connected to PipeWire.

# Returns

TRUE if the core is connected to PipeWire, FALSE otherwise
<!-- impl Core::fn load_component -->
Loads the specified `component` on `self`.


The `type_` will determine which component loader to use. The following types are built-in and will always work without a component loader:
 - "module" - Loads a WirePlumber module
 - "pw_module" - Loads a PipeWire module
## `component`
the module name or file name
## `type_`
the type of the component
## `args`
additional arguments for the component, usually a dict or a string

# Returns

TRUE if loaded, FALSE if there was an error
<!-- impl Core::fn sync -->
Asks the PipeWire server to call the `callback` via an event.


Since methods are handled in-order and events are delivered in-order, this can be used as a barrier to ensure all previous methods and the resulting events have been handled.
In both success and error cases, `callback` is always called. Use `wp_core_sync_finish()` from within the `callback` to determine whether the operation completed successfully or if an error occurred.
## `cancellable`
a GCancellable to cancel the operation
## `callback`
a function to call when the operation is done

# Returns

TRUE if the sync operation was started, FALSE if an error occurred before returning from this function
<!-- impl Core::fn sync_closure -->
Asks the PipeWire server to invoke the `closure` via an event.


Since methods are handled in-order and events are delivered in-order, this can be used as a barrier to ensure all previous methods and the resulting events have been handled.
In both success and error cases, `closure` is always invoked. Use `wp_core_sync_finish()` from within the `closure` to determine whether the operation completed successfully or if an error occurred.
## `cancellable`
a GCancellable to cancel the operation
## `closure`
a closure to invoke when the operation is done

# Returns

TRUE if the sync operation was started, FALSE if an error occurred before returning from this function
<!-- impl Core::fn timeout_add -->
Adds a timeout callback to be called at regular intervals in the same GMainContext as the one used by this core.


The function is called repeatedly until it returns FALSE, at which point the timeout is automatically destroyed and the function will not be called again. The first call to the function will be at the end of the first interval.
This is essentially the same as `g_timeout_add_full()`, but it adds the created GSource on the GMainContext used by this core instead of the default context.
## `timeout_ms`
the timeout in milliseconds
## `function`
the function to call

# Returns


## `source`
the source
<!-- impl Core::fn timeout_add_closure -->
Adds a timeout callback to be called at regular intervals in the same GMainContext as the one used by this core.


This is the same as [`timeout_add()`][Self::timeout_add()], but it allows you to specify a GClosure instead of a C callback.
## `timeout_ms`
the timeout in milliseconds
## `closure`
the closure to invoke

# Returns


## `source`
the source
<!-- impl Core::fn update_properties -->
Updates the properties of `self` on the connection, making them appear on the client object that represents this connection.


If `self` is not connected yet, these properties are stored and passed to `pw_context_connect()` when connecting.
## `updates`
updates to apply to the properties of `self`; this does not need to include properties that have not changed
<!-- struct Device -->


# Implements

[`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- impl Device::fn from_factory -->
Constructs a device on the PipeWire server by asking the remote factory `factory_name` to create it.


Because of the nature of the PipeWire protocol, this operation completes asynchronously at some point in the future. In order to find out when this is done, you should call [`ObjectExt::activate()`][crate::prelude::ObjectExt::activate()], requesting at least WP_PROXY_FEATURE_BOUND. When this feature is ready, the device is ready for use on the server. If the device cannot be created, this activation operation will fail.
## `core`
the wireplumber core
## `factory_name`
the pipewire factory name to construct the device
## `properties`
the properties to pass to the factory

# Returns

the new device or NULL if the core is not connected and therefore the device cannot be created
<!-- enum Direction -->
The different directions that a port can have.
<!-- struct Endpoint -->


# Implements

[`EndpointExt`][trait@crate::prelude::EndpointExt], [`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- trait EndpointExt -->
Trait containing all [`struct@Endpoint`] methods.

# Implementors

[`Endpoint`][struct@crate::Endpoint], [`ImplEndpoint`][struct@crate::ImplEndpoint]
<!-- trait EndpointExt::fn direction -->
Gets the direction of the endpoint.


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

the direction of this endpoint
<!-- trait EndpointExt::fn media_class -->
Gets the media class of the endpoint (ex. "Audio/Sink")


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

the media class of the endpoint
<!-- trait EndpointExt::fn name -->
Gets the name of the endpoint.


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

the name of the endpoint
<!-- struct Factory -->


# Implements

[`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- struct FeatureActivationTransition -->


# Implements

[`TransitionExt`][trait@crate::prelude::TransitionExt], [`trait@glib::ObjectExt`], [`trait@gio::prelude::AsyncResultExt`]
<!-- impl FeatureActivationTransition::fn requested_features -->
Gets the features requested to be activated in this transition.

# Returns

the features that were requested to be activated in this transition; this contains the features as they were passed in [`ObjectExt::activate()`][crate::prelude::ObjectExt::activate()] and therefore it may contain unsupported or already active features
<!-- struct GlobalProxy -->


# Implements

[`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait GlobalProxyExt -->
Trait containing all [`struct@GlobalProxy`] methods.

# Implementors

[`Client`][struct@crate::Client], [`Device`][struct@crate::Device], [`Endpoint`][struct@crate::Endpoint], [`Factory`][struct@crate::Factory], [`GlobalProxy`][struct@crate::GlobalProxy], [`Link`][struct@crate::Link], [`Metadata`][struct@crate::Metadata], [`Node`][struct@crate::Node], [`Port`][struct@crate::Port]
<!-- trait GlobalProxyExt::fn bind -->
Binds to the global and creates the underlying pw_proxy.


This is mostly meant to be called internally. It will create the pw_proxy and will activate the WP_PROXY_FEATURE_BOUND feature.
This may only be called if there is no pw_proxy associated with this object yet.

# Returns

TRUE on success, FALSE if there is no global to bind to
<!-- trait GlobalProxyExt::fn global_properties -->
Gets the global properties of a pipewire global.

# Returns

the global (immutable) properties of this pipewire object
<!-- trait GlobalProxyExt::fn permissions -->
Gets the permissions of a pipewire global.

# Returns

the permissions that wireplumber has on this object
<!-- struct ImplEndpoint -->


# Implements

[`EndpointExt`][trait@crate::prelude::EndpointExt], [`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- impl ImplEndpoint::fn new -->
Creates a new endpoint implementation.
## `core`
the core
## `item`
the session item that implements the endpoint

# Returns

a new WpImplEndpoint
<!-- struct ImplMetadata -->


# Implements

[`MetadataExt`][trait@crate::prelude::MetadataExt], [`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- impl ImplMetadata::fn new -->
Creates a new metadata implementation.
## `core`
the core

# Returns

a new WpImplMetadata
<!-- impl ImplMetadata::fn new_full -->
Creates a new metadata implementation with name and properties.
## `core`
the core
## `name`
the metadata name
## `properties`
the metadata properties

# Returns

a new WpImplMetadata
<!-- struct ImplModule -->


# Implements

[`trait@glib::ObjectExt`]
<!-- impl ImplModule::fn load -->
Loads a PipeWire module into the WirePlumber process.
## `core`
The WirePlumber core
## `name`
the name of the module to load
## `arguments`
arguments to be passed to the module
## `properties`
additional properties to be provided to the module

# Returns

the WpImplModule for the module that was loaded on success, NULL on failure.
<!-- struct ImplNode -->


# Implements

[`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- impl ImplNode::fn from_pw_factory -->
Constructs a new node, locally on this process, using the specified `factory_name`.


To export this node to the PipeWire server, you need to call [`ObjectExt::activate()`][crate::prelude::ObjectExt::activate()] requesting WP_PROXY_FEATURE_BOUND and wait for the operation to complete.
## `core`
the wireplumber core
## `factory_name`
the name of the pipewire factory
## `properties`
properties to be passed to node constructor

# Returns

A new WpImplNode wrapping the node that was constructed by the factory, or NULL if the factory does not exist or was unable to construct the node
<!-- impl ImplNode::fn new_wrap -->
Constructs a node object from an existing pw_impl_node.
## `core`
the wireplumber core
## `node`
an existing pw_impl_node to wrap

# Returns

A new WpImplNode wrapping `node`
<!-- struct InitFlags -->
<!-- struct InterestMatch -->
Flags that indicate which constraints have been matched in [`ObjectInterest::matches_full()`][crate::ObjectInterest::matches_full()]
<!-- struct InterestMatchFlags -->
Flags to alter the behaviour of [`ObjectInterest::matches_full()`][crate::ObjectInterest::matches_full()]
<!-- struct Iterator -->
<!-- impl Iterator::fn new_ptr_array -->
Creates an iterator from a pointer array.
## `items`
the items to iterate over
## `item_type`
the type of each item

# Returns

a new iterator that iterates over `items`
<!-- impl Iterator::fn fold -->
Fold a function over the items of the iterator.
## `func`
the fold function
## `ret`
the accumulator data

# Returns

TRUE if all the items were processed, FALSE otherwise.
<!-- impl Iterator::fn foreach -->
Iterates over all items of the iterator calling a function.
## `func`
the foreach function

# Returns

TRUE if all the items were processed, FALSE otherwise.
<!-- impl Iterator::fn user_data -->
Gets the implementation-specific storage of an iterator.


this only for use by implementations of WpIterator

# Returns

a pointer to the implementation-specific storage area
<!-- impl Iterator::fn next -->
Gets the next item of the iterator.

# Returns

TRUE if next iterator was obtained, FALSE when the iterator has no more items to iterate through.

## `item`
the next item of the iterator
<!-- impl Iterator::fn reset -->
Resets the iterator so we can iterate again from the beginning.
<!-- struct IteratorMethods -->
<!-- enum LibraryErrorEnum -->
Error codes that can appear in a GError when the error domain is WP_DOMAIN_LIBRARY.
<!-- struct Link -->


# Implements

[`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- impl Link::fn from_factory -->
Constructs a link on the PipeWire server by asking the remote factory `factory_name` to create it.


Because of the nature of the PipeWire protocol, this operation completes asynchronously at some point in the future. In order to find out when this is done, you should call [`ObjectExt::activate()`][crate::prelude::ObjectExt::activate()], requesting at least WP_PROXY_FEATURE_BOUND. When this feature is ready, the link is ready for use on the server. If the link cannot be created, this activation operation will fail.
## `core`
the wireplumber core
## `factory_name`
the pipewire factory name to construct the link
## `properties`
the properties to pass to the factory

# Returns

the new link or NULL if the core is not connected and therefore the link cannot be created
<!-- impl Link::fn linked_object_ids -->
Retrieves the ids of the objects that are linked by this link.


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns


## `output_node`
the bound id of the output (source) node

## `output_port`
the bound id of the output (source) port

## `input_node`
the bound id of the input (sink) node

## `input_port`
the bound id of the input (sink) port
<!-- struct LookupDirs -->
Flags to specify lookup directories.
<!-- struct Metadata -->


# Implements

[`MetadataExt`][trait@crate::prelude::MetadataExt], [`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait MetadataExt -->
Trait containing all [`struct@Metadata`] methods.

# Implementors

[`ImplMetadata`][struct@crate::ImplMetadata], [`Metadata`][struct@crate::Metadata]
<!-- impl Metadata::fn iterator_item_extract -->
Extracts the metadata subject, key, type and value out of a GValue that was returned from the WpIterator of [`MetadataExt::find()`][crate::prelude::MetadataExt::find()]
## `item`
a GValue that was returned from the WpIterator of [`MetadataExt::find()`][crate::prelude::MetadataExt::find()]

# Returns


## `subject`
the subject id of the current item

## `key`
the key of the current item

## `type_`
the type of the current item

## `value`
the value of the current item
<!-- trait MetadataExt::fn clear -->
Clears permanently all stored metadata.
<!-- trait MetadataExt::fn find -->
Finds the metadata value given its `subject` and `key`.
## `subject`
the metadata subject id
## `key`
the metadata key name

# Returns

the metadata string value, or NULL if not found.

## `type_`
the metadata type name
<!-- trait MetadataExt::fn new_iterator -->
Iterates over metadata items that matches the given `subject`.


If no constraints are specified, the returned iterator iterates over all the stored metadata.
Note that this method works on cached metadata. When you change metadata with [`set()`][Self::set()], this cache will be updated on the next round-trip with the pipewire server.
## `subject`
the metadata subject id, or -1 (PW_ID_ANY)

# Returns

an iterator that iterates over the found metadata. Use [`Metadata::iterator_item_extract()`][crate::Metadata::iterator_item_extract()] to parse the items returned by this iterator.
<!-- trait MetadataExt::fn set -->
Sets the metadata associated with the given `subject` and `key`. Use NULL as a value to unset the given `key` and use NULL in both `key` and `value` to remove all metadata associated with the given `subject`.
## `subject`
the subject id for which this metadata property is being set
## `key`
the key to set, or NULL to remove all metadata for `subject`
## `type_`
the type of the value; NULL is synonymous to "string"
## `value`
the value to set, or NULL to unset the given `key`
<!-- struct MetadataFeatures -->
An extension of WpProxyFeatures for WpMetadata objects.
<!-- struct Node -->


# Implements

[`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- impl Node::fn from_factory -->
Constructs a node on the PipeWire server by asking the remote factory `factory_name` to create it.


Because of the nature of the PipeWire protocol, this operation completes asynchronously at some point in the future. In order to find out when this is done, you should call [`ObjectExt::activate()`][crate::prelude::ObjectExt::activate()], requesting at least WP_PROXY_FEATURE_BOUND. When this feature is ready, the node is ready for use on the server. If the node cannot be created, this activation operation will fail.
## `core`
the wireplumber core
## `factory_name`
the pipewire factory name to construct the node
## `properties`
the properties to pass to the factory

# Returns

the new node or NULL if the core is not connected and therefore the node cannot be created
<!-- impl Node::fn n_input_ports -->
Gets the number of input ports of this node.


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

the number of input ports of this node, as reported by the node info

## `max`
the maximum supported number of input ports
<!-- impl Node::fn n_output_ports -->
Gets the number of output ports of this node.


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

the number of output ports of this node, as reported by the node info

## `max`
the maximum supported number of output ports
<!-- impl Node::fn n_ports -->
Gets the number of ports of this node.


Note that this number may not add up to [`n_input_ports()`][Self::n_input_ports()] + [`n_output_ports()`][Self::n_output_ports()] because it is discovered by looking at the number of available ports in the registry, however ports may appear there with a delay or may not appear at all if this client does not have permission to read them
Requires WP_NODE_FEATURE_PORTS

# Returns

the number of ports of this node.
<!-- impl Node::fn state -->
Gets the current state of the node.

# Returns

the current state of the node

## `error`
the error
<!-- impl Node::fn lookup_port -->
Retrieves the first port that matches the constraints.


The constraints specified in the variable arguments must follow the rules documented in `wp_object_interest_new()`.
Requires WP_NODE_FEATURE_PORTS

# Returns

the first port that matches the constraints, or NULL if there is no such port
<!-- impl Node::fn lookup_port_full -->
Retrieves the first port that matches the `interest`.


Requires WP_NODE_FEATURE_PORTS
## `interest`
the interest

# Returns

the first port that matches the `interest`, or NULL if there is no such port
<!-- impl Node::fn new_ports_filtered_iterator -->
Gets a new iterator that iterates over all the ports that belong to this node and match the constraints.


The constraints specified in the variable arguments must follow the rules documented in `wp_object_interest_new()`.
Requires WP_NODE_FEATURE_PORTS

# Returns

a WpIterator that iterates over WpPort objects
<!-- impl Node::fn new_ports_filtered_iterator_full -->
Gets a new iterator that iterates over all the ports that belong to this node and match the `interest`.


Requires WP_NODE_FEATURE_PORTS
## `interest`
the interest

# Returns

a WpIterator that iterates over WpPort objects
<!-- impl Node::fn new_ports_iterator -->
Gets a new iterator that iterates over all the ports that belong to this node.


Requires WP_NODE_FEATURE_PORTS

# Returns

a WpIterator that iterates over WpPort objects
<!-- impl Node::fn send_command -->
Sends a command to a node.


Valid commands are the short string reprepsentations of enum spa_node_command. For example, "Suspend" or "Flush" are valid commands
## `command`
the command
<!-- struct NodeFeatures -->
An extension of WpProxyFeatures.
<!-- enum NodeState -->
The state of the node.
<!-- struct Object -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait ObjectExt -->
Trait containing all [`struct@Object`] methods.

# Implementors

[`Object`][struct@crate::Object], [`Plugin`][struct@crate::Plugin], [`Proxy`][struct@crate::Proxy], [`SessionItem`][struct@crate::SessionItem]
<!-- trait ObjectExt::fn abort_activation -->
Aborts the current object activation by returning a transition error if any transitions are pending.


This is usually used to stop any pending activation if an error happened.
## `msg`
the message used in the transition error
<!-- trait ObjectExt::fn activate -->
Callback version of [`activate_closure()`][Self::activate_closure()]
## `features`
the features to enable
## `cancellable`
a cancellable for the async operation
## `callback`
a function to call when activation is complete
<!-- trait ObjectExt::fn activate_closure -->
Activates the requested `features` and invokes `closure` when this is done. `features` may contain unsupported or already active features. The operation will filter them and activate only ones that are supported and inactive.


If multiple calls to this method is done, the operations will be executed one after the other to ensure features only get activated once.
`closure` may be invoked in sync while this method is being called, if there are no features to activate.
## `features`
the features to enable
## `cancellable`
a cancellable for the async operation
## `closure`
the closure to use when activation is completed
<!-- trait ObjectExt::fn deactivate -->
Deactivates the given `features`, leaving the object in the state it was before they were enabled.


This is seldom needed to call manually, but it can be used to save resources if some features are no longer needed.
## `features`
the features to deactivate
<!-- trait ObjectExt::fn active_features -->
Gets the active features of this object.

# Returns

A bitset containing the active features of this object
<!-- trait ObjectExt::fn core -->
Gets the core associated with this object.

# Returns

the core associated with this object
<!-- trait ObjectExt::fn supported_features -->
Gets the supported features of this object.

# Returns

A bitset containing the supported features of this object; note that supported features may change at runtime
<!-- trait ObjectExt::fn update_features -->
Allows subclasses to update the currently active features.


`activated` should contain new features and `deactivated` should contain features that were just deactivated. Calling this method also advances the activation transitions.
Private method to be called by subclasses only.
## `activated`
the features that were activated, or 0
## `deactivated`
the features that were deactivated, or 0
<!-- struct ObjectInterest -->
<!-- impl ObjectInterest::fn new -->
Creates a new interest that declares interest in objects of the specified `gtype`, with the constraints specified in the variable arguments.


The variable arguments should be a list of constraints terminated with NULL, where each constraint consists of the following arguments:
 - a WpConstraintType: the constraint type
 - a const gchar *: the subject name
 - a const gchar *: the format string
 - 0 or more arguments according to the format string

The format string is interpreted as follows:
 - the first character is the constraint verb:
 - =: WP_CONSTRAINT_VERB_EQUALS
 - !: WP_CONSTRAINT_VERB_NOT_EQUALS
 - c: WP_CONSTRAINT_VERB_IN_LIST
 - ~: WP_CONSTRAINT_VERB_IN_RANGE
 - #: WP_CONSTRAINT_VERB_MATCHES
 - +: WP_CONSTRAINT_VERB_IS_PRESENT
 - -: WP_CONSTRAINT_VERB_IS_ABSENT

 - the rest of the characters are interpreted as a GVariant format string, as it would be used in [`glib::Variant::new()`][crate::glib::Variant::new()]

The rest of this function's arguments up to the start of the next constraint depend on the GVariant format part of the format string and are used to construct a GVariant for the constraint's value argument.
For further reading on the constraint's arguments, see [`add_constraint()`][Self::add_constraint()]
For example, this interest matches objects that are descendands of WpProxy with a "bound-id" between 0 and 100 (inclusive), with a pipewire property called "format.dsp" that contains the string "audio" somewhere in the value and with a pipewire property "port.name" being present (with any value):


**⚠️ The following code is in C ⚠️**

```C
  interest = wp_object_interest_new (WP_TYPE_PROXY,
      WP_CONSTRAINT_TYPE_G_PROPERTY, "bound-id", "~(uu)", 0, 100,
      WP_CONSTRAINT_TYPE_PW_PROPERTY, "format.dsp", "#s", "*audio*",
      WP_CONSTRAINT_TYPE_PW_PROPERTY, "port.name", "+",
      NULL);
```
## `gtype`
the type of the object to declare interest in

# Returns

the new object interest
<!-- impl ObjectInterest::fn new_type -->
Creates a new interest that declares interest in objects of the specified `gtype`, without any property constraints.


To add property constraints, you can call [`add_constraint()`][Self::add_constraint()] afterwards.
## `gtype`
the type of the object to declare interest in

# Returns

the new object interest
<!-- impl ObjectInterest::fn new_valist -->
va_list version of `wp_object_interest_new()`
## `gtype`
the type of the object to declare interest in
## `args`
pointer to va_list containing the constraints

# Returns

the new object interest
<!-- impl ObjectInterest::fn add_constraint -->
Adds a constraint to this interest. Constraints consist of a `type_`, a `subject`, a `verb` and, depending on the `verb`, a `value`.


Constraints are almost like a spoken language sentence that declare a condition that must be true in order to consider that an object can match this interest. For instance, a constraint can be "pipewire property
'object.id' equals 10". This would be translated to:


**⚠️ The following code is in C ⚠️**

```C
  wp_object_interest_add_constraint (i,
     WP_CONSTRAINT_TYPE_PW_PROPERTY, "object.id",
     WP_CONSTRAINT_VERB_EQUALS, g_variant_new_int (10));
```
Some verbs require a `value` and some others do not. For those that do, the `value` must be of a specific type:
 - WP_CONSTRAINT_VERB_EQUALS: `value` can be a string, a (u)int32, a (u)int64, a double or a boolean. The `subject` value must equal this value for the constraint to be satisfied
 - WP_CONSTRAINT_VERB_IN_LIST: `value` must be a tuple that contains any number of items of the same type; the items can be string, (u)int32, (u)int64 or double. These items make a list that the `subject`'s value will be checked against. If any of the items equals the `subject` value, the constraint is satisfied
 - WP_CONSTRAINT_VERB_IN_RANGE: `value` must be a tuple that contains exactly 2 numbers of the same type ((u)int32, (u)int64 or double), meaning the minimum and maximum (inclusive) of the range. If the `subject` value is a number within this range, the constraint is satisfied
 - WP_CONSTRAINT_VERB_MATCHES: `value` must be a string that defines a pattern usable with GPatternSpec If the `subject` value matches this pattern, the constraint is satisfied

In case the type of the `subject` value is not the same type as the one requested by the type of the `value`, the `subject` value is converted. For GObject properties, this conversion is done using [`glib::Value::transform()`][crate::glib::Value::transform()], so limitations of this function apply. In the case of PipeWire properties, which are `always` strings, conversion is done as follows:
 - to boolean: "true" or "1" means TRUE, "false" or "0" means FALSE
 - to int / uint / int64 / uint64: One of the `strtol()` family of functions is used to convert, using base 10
 - to double: `strtod()` is used

This method does not fail if invalid arguments are given. However, [`validate()`][Self::validate()] should be called after adding all the constraints on an interest in order to catch errors.
## `type_`
the constraint type
## `subject`
the subject that the constraint applies to
## `verb`
the operation that is performed to check the constraint
## `value`
the value to check for
<!-- impl ObjectInterest::fn matches -->
Checks if the specified `object` matches the type and all the constraints that are described in `self`.


If `self` is configured to match GObject subclasses, this is equivalent to wp_object_interest_matches_full (self, G_OBJECT_TYPE (object), object, NULL, NULL) and if it is configured to match WpProperties, this is equivalent to wp_object_interest_matches_full (self, self->gtype, NULL, (WpProperties *) object, NULL);
## `object`
the target object to check for a match

# Returns

TRUE if the object matches, FALSE otherwise
<!-- impl ObjectInterest::fn matches_full -->
A low-level version of `wp_object_interest_matches()`.


In this version, the object's type is directly given in `object_type` and is not inferred from the `object`. `object` is only used to check for constraints against GObject properties.
`pw_props` and `pw_global_props` are used to check constraints against PipeWire object properties and global properties, respectively.
`object`, `pw_props` and `pw_global_props` may be NULL, but in case there are any constraints that require them, the match will fail. As a special case, if `object` is not NULL and is a subclass of WpProxy, then `pw_props` and `pw_global_props`, if required, will be internally retrieved from `object` by calling [`PipewireObjectExt::properties()`][crate::prelude::PipewireObjectExt::properties()] and [`GlobalProxyExt::global_properties()`][crate::prelude::GlobalProxyExt::global_properties()] respectively.
When `flags` contains WP_INTEREST_MATCH_FLAGS_CHECK_ALL, all the constraints are checked and the returned value contains accurate information about which types of constraints have failed to match, if any. When this flag is not present, this function returns after the first failure has been encountered. This means that the returned flags set will contain all but one flag, which will indicate the kind of constraint that failed (more could have failed, but they are not checked...)
## `flags`
flags to alter the behavior of this function
## `object_type`
the type to be checked against the interest's type
## `object`
the object to be used for checking constraints of type WP_CONSTRAINT_TYPE_G_PROPERTY
## `pw_props`
the properties to be used for checking constraints of type WP_CONSTRAINT_TYPE_PW_PROPERTY
## `pw_global_props`
the properties to be used for checking constraints of type WP_CONSTRAINT_TYPE_PW_GLOBAL_PROPERTY

# Returns

flags that indicate which components of the interest match. WP_INTEREST_MATCH_ALL indicates a fully successful match; any other combination indicates a failure on the component(s) that do not appear on the flag set
<!-- impl ObjectInterest::fn validate -->
Validates the interest, ensuring that the interest GType is a valid object and that all the constraints have been expressed properly.


This is called internally when `self` is first used to find a match, so it is not necessary to call it explicitly

# Returns

TRUE if the interest is valid and can be used in a match, FALSE otherwise
<!-- struct ObjectManager -->


# Implements

[`trait@glib::ObjectExt`]
<!-- impl ObjectManager::fn new -->
Constructs a new object manager.

# Returns

the newly constructed object manager
<!-- impl ObjectManager::fn add_interest -->
Equivalent to:





**⚠️ The following code is in C ⚠️**

```C
  WpObjectInterest *i = wp_object_interest_new (gtype, ...);
  wp_object_manager_add_interest_full (self, i);
```
The constraints specified in the variable arguments must follow the rules documented in `wp_object_interest_new()`.
## `gtype`
the GType of the objects that we are declaring interest in
<!-- impl ObjectManager::fn add_interest_full -->
Declares interest in a certain kind of object.


Interest consists of a GType that the object must be an ancestor of (`g_type_is_a()` must match) and optionally, a set of additional constraints on certain properties of the object. Refer to WpObjectInterest for more details.
## `interest`
the interest
<!-- impl ObjectManager::fn n_objects -->
Gets the number of objects managed by the object manager.

# Returns

the number of objects managed by this WpObjectManager
<!-- impl ObjectManager::fn is_installed -->
Checks if an object manager is installed.

# Returns

TRUE if the object manager is installed (i.e. the WpObjectManager installed signal has been emitted), FALSE otherwise
<!-- impl ObjectManager::fn lookup -->
Equivalent to:





**⚠️ The following code is in C ⚠️**

```C
  WpObjectInterest *i = wp_object_interest_new (gtype, ...);
  return wp_object_manager_lookup_full (self, i);
```
The constraints specified in the variable arguments must follow the rules documented in `wp_object_interest_new()`.
## `gtype`
the GType of the object to lookup

# Returns

the first managed object that matches the lookup interest, or NULL if no object matches
<!-- impl ObjectManager::fn lookup_full -->
Searches for an object that matches the specified `interest` and returns it, if found.


If more than one objects match, only the first one is returned. To find multiple objects that match certain criteria, `wp_object_manager_new_filtered_iterator()` is more suitable.
## `interest`
the interst

# Returns

the first managed object that matches the lookup interest, or NULL if no object matches
<!-- impl ObjectManager::fn new_filtered_iterator -->
Equivalent to:





**⚠️ The following code is in C ⚠️**

```C
  WpObjectInterest *i = wp_object_interest_new (gtype, ...);
  return wp_object_manager_new_filtered_iterator_full (self, i);
```
The constraints specified in the variable arguments must follow the rules documented in `wp_object_interest_new()`.
## `gtype`
the GType of the objects to iterate through

# Returns

a WpIterator that iterates over all the matching objects of this object manager
<!-- impl ObjectManager::fn new_filtered_iterator_full -->
Iterates through all the objects managed by this object manager that match the specified `interest`.
## `interest`
the interest

# Returns

a WpIterator that iterates over all the matching objects of this object manager
<!-- impl ObjectManager::fn new_iterator -->
Iterates through all the objects managed by this object manager.

# Returns

a WpIterator that iterates over all the managed objects of this object manager
<!-- impl ObjectManager::fn request_object_features -->
Requests the object manager to automatically prepare the `wanted_features` on any managed object that is of the specified `object_type`.


These features will always be prepared before the object appears on the object manager.
## `object_type`
the WpProxy descendant type
## `wanted_features`
the features to enable on this kind of object
<!-- struct PipewireObject -->


# Implements

[`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait PipewireObjectExt -->
Trait containing all [`struct@PipewireObject`] methods.

# Implementors

[`Client`][struct@crate::Client], [`Device`][struct@crate::Device], [`Endpoint`][struct@crate::Endpoint], [`Factory`][struct@crate::Factory], [`ImplEndpoint`][struct@crate::ImplEndpoint], [`ImplNode`][struct@crate::ImplNode], [`Link`][struct@crate::Link], [`Node`][struct@crate::Node], [`PipewireObject`][struct@crate::PipewireObject], [`Port`][struct@crate::Port]
<!-- trait PipewireObjectExt::fn enum_params -->
Enumerate object parameters.


This will asynchronously return the result, or an error, by calling the given `callback`. The result is going to be a WpIterator containing WpSpaPod objects, which can be retrieved with `wp_pipewire_object_enum_params_finish()`.
## `id`
the parameter id to enumerate or NULL for all parameters
## `filter`
a param filter or NULL
## `cancellable`
a cancellable for the async operation
## `callback`
a callback to call with the result
<!-- trait PipewireObjectExt::fn enum_params_sync -->
This method can be used to retrieve object parameters in a synchronous way (in contrast with [`enum_params()`][Self::enum_params()], which is async).


The WP_PIPEWIRE_OBJECT_FEATURE_PARAM_`<something>` feature that corresponds to the specified `id` must have been activated earlier. These features enable monitoring and caching of params underneath, so that they are always available for retrieval with this method.
Note, however, that cached params may be out-of-date if they have changed very recently on the remote object and the caching mechanism hasn't been able to update them yet, so if you really need up-to-date information you should only rely on [`enum_params()`][Self::enum_params()] instead.
## `id`
the parameter id to enumerate
## `filter`
a param filter or NULL

# Returns

an iterator to iterate over cached parameters, or NULL if parameters for this `id` are not cached; the items in the iterator are WpSpaPod
<!-- trait PipewireObjectExt::fn native_info -->
Retrieves the native infor structure of this object (pw_node_info, pw_port_info, etc...)


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

the native pipewire info structure of this object
<!-- trait PipewireObjectExt::fn param_info -->
Returns the available parameters of this pipewire object.


The return value is a GVariant of type a{ss}, where the key of each map entry is a spa param type id (the same ids that you can pass in [`enum_params()`][Self::enum_params()]) and the value is a string that can contain the following letters, each of them representing a flag:
 - r: the param is readable (SPA_PARAM_INFO_READ)
 - w: the param is writable (SPA_PARAM_INFO_WRITE)

For params that are readable, you can query them with [`enum_params()`][Self::enum_params()]
Params that are writable can be set with [`set_param()`][Self::set_param()]
Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

a variant of type a{ss} or NULL if the object does not support params at all
<!-- trait PipewireObjectExt::fn properties -->
Retrieves the PipeWire properties of this object.


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

the pipewire properties of this object; normally these are the properties that are part of the info structure
<!-- trait PipewireObjectExtManual::fn property -->
Returns the value of a single pipewire property.


This is the same as getting the whole properties structure with [`PipewireObjectExt::properties()`][crate::prelude::PipewireObjectExt::properties()] and accessing a single property with [`Properties::get()`][crate::Properties::get()], but saves one call and having to clean up the WpProperties reference count afterwards.
The value is owned by the proxy, but it is guaranteed to stay alive until execution returns back to the event loop.
Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO
## `key`
the property name

# Returns

the value of the pipewire property `key` or NULL if the property doesn't exist
<!-- trait PipewireObjectExt::fn new_properties_iterator -->
Iterates over the object's PipeWire properties.


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

an iterator that iterates over the pipewire properties of this object. Use [`Properties::iterator_item_get_key()`][crate::Properties::iterator_item_get_key()] and [`Properties::iterator_item_get_value()`][crate::Properties::iterator_item_get_value()] to parse the items returned by this iterator.
<!-- trait PipewireObjectExt::fn set_param -->
Sets a parameter on the object.
## `id`
the parameter id to set
## `flags`
optional flags or 0
## `param`
the parameter to set

# Returns

TRUE on success, FALSE if setting the param failed
<!-- struct Plugin -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`PluginExt`][trait@crate::prelude::PluginExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait PluginExt -->
Trait containing all [`struct@Plugin`] methods.

# Implementors

[`ComponentLoader`][struct@crate::ComponentLoader], [`Plugin`][struct@crate::Plugin]
<!-- impl Plugin::fn find -->
Looks up a plugin.
## `core`
the core
## `plugin_name`
the lookup name

# Returns

the plugin matching the lookup name
<!-- trait PluginExt::fn name -->
Retreives the name of a plugin.

# Returns

the name of this plugin
<!-- trait PluginExt::fn register -->
Registers the plugin to its associated core, making it available for use.
<!-- struct PluginFeatures -->
Flags to be used as WpObjectFeatures on WpPlugin subclasses.
<!-- struct Port -->


# Implements

[`GlobalProxyExt`][trait@crate::prelude::GlobalProxyExt], [`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`], [`PipewireObjectExt`][trait@crate::prelude::PipewireObjectExt]
<!-- impl Port::fn direction -->
Gets the current direction of the port.


Requires WP_PIPEWIRE_OBJECT_FEATURE_INFO

# Returns

the current direction of the port
<!-- struct Properties -->
<!-- impl Properties::fn new -->
Constructs a new properties set that contains the given properties.
## `key`
a property name

# Returns

the newly constructed properties set
<!-- impl Properties::fn new_copy_dict -->
Constructs a new WpProperties that contains a copy of all the properties contained in the given `dict` structure.
## `dict`
a native spa_dict structure to copy

# Returns

the newly constructed properties set
<!-- impl Properties::fn new_empty -->
Creates a new empty properties set.

# Returns

the newly constructed properties set
<!-- impl Properties::fn new_string -->
Constructs a new properties set that contains the properties that can be parsed from the given string.
## `str`
a string containing a whitespace separated list of key=value pairs (ex. "key1=value1 key2=value2")

# Returns

the newly constructed properties set
<!-- impl Properties::fn new_take -->
Constructs a new WpProperties that wraps the given `props` structure, allowing reading & writing properties on that `props` structure through the WpProperties API.


In constrast with `wp_properties_new_wrap()`, this function assumes ownership of the `props` structure, so it will try to free `props` when it is destroyed.
## `props`
a native pw_properties structure to wrap

# Returns

the newly constructed properties set
<!-- impl Properties::fn new_valist -->
This is the va_list version of `wp_properties_new()`
## `key`
a property name
## `args`
the variable arguments passed to `wp_properties_new()`

# Returns

the newly constructed properties set
<!-- impl Properties::fn new_wrap -->
Constructs a new WpProperties that wraps the given `props` structure, allowing reading properties on that `props` structure through the WpProperties API.


Care must be taken when using this function, since the returned WpProperties object does not own the `props` structure. Therefore, if the owner decides to free `props`, the returned WpProperties will crash when used. In addition, the returned WpProperties object will not try to free `props` when destroyed.
Furthermore, note that the returned WpProperties object is immutable. That means that you cannot add or modify any properties on it, unless you make a copy first.
## `props`
a native pw_properties structure to wrap

# Returns

the newly constructed properties set
<!-- impl Properties::fn new_wrap_dict -->
Constructs a new WpProperties that wraps the given `dict` structure, allowing reading properties from that `dict` through the WpProperties API.


Note that the returned object does not own the `dict`, so care must be taken not to free it externally while this WpProperties object is alive.
In addition, note that the returned WpProperties object is immutable. That means that you cannot add or modify any properties on it, since there is no defined method for modifying a struct spa_dict. If you need to change this properties set later, you should make a copy with `wp_properties_copy()`.
## `dict`
a native spa_dict structure to wrap

# Returns

the newly constructed properties set
<!-- impl Properties::fn add -->
Adds new properties in `self`, using the given `props` as a source.


Properties (keys) from `props` that are already contained in `self` are not modified, unlike what happens with [`update()`][Self::update()]. Properties in `self` that are not contained in `props` are left untouched.
## `props`
a properties set that contains properties to add

# Returns

the number of properties that were changed
<!-- impl Properties::fn add_from_dict -->
Adds new properties in `self`, using the given `dict` as a source.


Properties (keys) from `dict` that are already contained in `self` are not modified, unlike what happens with `wp_properties_update_from_dict()`. Properties in `self` that are not contained in `dict` are left untouched.
## `dict`
a spa_dict that contains properties to add

# Returns

the number of properties that were changed
<!-- impl Properties::fn add_keys -->
Adds new properties in `self`, using the given `props` as a source.


Unlike [`add()`][Self::add()], this function only adds properties that have one of the specified keys; the rest is left untouched.
## `props`
a properties set that contains properties to add
## `key1`
a property to add

# Returns

the number of properties that were changed
<!-- impl Properties::fn add_keys_array -->
The same as `wp_properties_add_keys()`, using a NULL-terminated array for specifying the keys to add.
## `props`
a properties set that contains properties to add
## `keys`
the properties to add

# Returns

the number of properties that were changed
<!-- impl Properties::fn add_keys_from_dict -->
Adds new properties in `self`, using the given `dict` as a source.


Unlike `wp_properties_add_from_dict()`, this function only adds properties that have one of the specified keys; the rest is left untouched.
## `dict`
a spa_dict that contains properties to add
## `key1`
a property to add

# Returns

the number of properties that were changed
<!-- impl Properties::fn ensure_unique_owner -->
Ensures that the given properties set is uniquely owned.


"Uniquely owned" means that:
 - its reference count is 1
 - it is not wrapping a native spa_dict or pw_properties object

If `self` is not uniquely owned already, then it is unrefed and a copy of it is returned instead. You should always consider `self` as unsafe to use after this call and you should use the returned object instead.

# Returns

the uniquely owned properties object
<!-- impl Properties::fn get -->
Looks up a given property value from a key.
## `key`
a property key

# Returns

the value of the property identified with `key`, or NULL if this property is not contained in `self`
<!-- impl Properties::fn matches -->
Checks if all property values contained in `other` are matching with the values in `self`.


If a property is contained in `other` and not in `self`, the result is not matched. If a property is contained in both sets, then the value of the property in `other` is interpreted as a glob-style pattern (using `g_pattern_match_simple()`) and the value in `self` is checked to see if it matches with this pattern.
## `other`
a set of properties to match

# Returns

TRUE if all matches were successfull, FALSE if at least one property value did not match
<!-- impl Properties::fn new_iterator -->
Iterates through all the properties in the properties object.

# Returns

an iterator that iterates over the properties. The items in the iterator are of type WpPropertiesItem. Use [`PropertiesItem::key()`][crate::PropertiesItem::key()] and [`PropertiesItem::value()`][crate::PropertiesItem::value()] to retrieve their contents.
<!-- impl Properties::fn peek_dict -->
Gets the dictionary wrapped by a properties object.

# Returns

the internal properties set as a struct spa_dict *
<!-- impl Properties::fn set -->
Sets the given property `key` - `value` pair on `self`.


If the property already existed, the value is overwritten with the new one.
If the `value` is NULL, then the specified property is removed from `self`
## `key`
a property key
## `value`
a property value

# Returns

1 if the property was changed. 0 if nothing was changed because the property already existed with the same value or because the key to remove did not exist.
<!-- impl Properties::fn setf -->
Formats the given `format` string with the specified arguments and sets the result as a value of the property specified with `key`.
## `key`
a property key
## `format`
a printf-style format to be formatted and set as a value for this property `key`

# Returns

1 if the property was changed. 0 if nothing was changed because the property already existed with the same value
<!-- impl Properties::fn setf_valist -->
This is the va_list version of `wp_properties_setf()`
## `key`
a property key
## `format`
a printf-style format to be formatted and set as a value for this property `key`
## `args`
the variable arguments passed to `wp_properties_setf()`

# Returns

1 if the property was changed. 0 if nothing was changed because the property already existed with the same value
<!-- impl Properties::fn sort -->
Sorts the keys in alphabetical order.
<!-- impl Properties::fn to_pw_properties -->
Gets a copy of the properties object as a struct pw_properties

# Returns

a copy of the properties in `self` as a struct pw_properties
<!-- impl Properties::fn unref_and_take_pw_properties -->
Similar to `wp_properties_to_pw_properties()`, but this method avoids making a copy of the properties by returning the struct pw_properties that is stored internally and then freeing the WpProperties wrapper.


If `self` is not uniquely owned (see [`ensure_unique_owner()`][Self::ensure_unique_owner()]), then this method does make a copy and is the same as `wp_properties_to_pw_properties()`, performance-wise.

# Returns

the properties in `self` as a struct pw_properties
<!-- impl Properties::fn update -->
Updates (adds new or modifies existing) properties in `self`, using the given `props` as a source.


Any properties that are not contained in `props` are left untouched.
## `props`
a properties set that contains properties to update

# Returns

the number of properties that were changed
<!-- impl Properties::fn update_from_dict -->
Updates (adds new or modifies existing) properties in `self`, using the given `dict` as a source.


Any properties that are not contained in `dict` are left untouched.
## `dict`
a spa_dict that contains properties to update

# Returns

the number of properties that were changed
<!-- impl Properties::fn update_keys -->
Updates (adds new or modifies existing) properties in `self`, using the given `props` as a source.


Unlike [`update()`][Self::update()], this function only updates properties that have one of the specified keys; the rest is left untouched.
## `props`
a properties set that contains properties to update
## `key1`
a property to update

# Returns

the number of properties that were changed
<!-- impl Properties::fn update_keys_array -->
The same as `wp_properties_update_keys()`, using a NULL-terminated array for specifying the keys to update.
## `props`
a properties set that contains properties to update
## `keys`
the properties to update

# Returns

the number of properties that were changed
<!-- impl Properties::fn update_keys_from_dict -->
Updates (adds new or modifies existing) properties in `self`, using the given `dict` as a source.


Unlike `wp_properties_update_from_dict()`, this function only updates properties that have one of the specified keys; the rest is left untouched.
## `dict`
a spa_dict that contains properties to update
## `key1`
a property to update

# Returns

the number of properties that were changed
<!-- impl Properties::fn iterator_item_get_key -->
Gets the key from a properties iterator item.

# Deprecated

Use [`PropertiesItem::key()`][crate::PropertiesItem::key()] instead
## `item`
a GValue that was returned from the WpIterator of [`new_iterator()`][Self::new_iterator()]

# Returns

the property key of the `item`
<!-- impl Properties::fn iterator_item_get_value -->
Gets the value from a properties iterator item.

# Deprecated

Use [`PropertiesItem::value()`][crate::PropertiesItem::value()] instead
## `item`
a GValue that was returned from the WpIterator of [`new_iterator()`][Self::new_iterator()]

# Returns

the property value of the `item`
<!-- struct PropertiesItem -->
<!-- impl PropertiesItem::fn key -->
Gets the key from a properties item.

# Returns

the property key of the `item`
<!-- impl PropertiesItem::fn value -->
Gets the value from a properties item.

# Returns

the property value of the `item`
<!-- struct Proxy -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait ProxyExt -->
Trait containing all [`struct@Proxy`] methods.

# Implementors

[`GlobalProxy`][struct@crate::GlobalProxy], [`ImplNode`][struct@crate::ImplNode], [`PipewireObject`][struct@crate::PipewireObject], [`Proxy`][struct@crate::Proxy], [`SpaDevice`][struct@crate::SpaDevice]
<!-- trait ProxyExt::fn bound_id -->
Returns the proxy bound id.


The bound id is the id that this object has on the PipeWire registry (a.k.a. the global id). The object must have the WP_PROXY_FEATURE_BOUND feature before this method can be called.
Requires WP_PROXY_FEATURE_BOUND

# Returns

the bound id of this object
<!-- trait ProxyExt::fn interface_type -->
Gets the interface type of the proxied object.

# Returns

the PipeWire type of the interface that is being proxied

## `version`
the version of the interface
<!-- trait ProxyExt::fn pw_proxy -->
Gets the pw_proxy wrapped by this proxy object.

# Returns

a pointer to the underlying pw_proxy object
<!-- trait ProxyExt::fn set_pw_proxy -->
Private method to be used by subclasses to set the pw_proxy pointer when it is available.


This can be called only if there is no pw_proxy already set. Takes ownership of `proxy`.
<!-- struct ProxyFeatures -->
Flags to be used as WpObjectFeatures for WpProxy subclasses.
<!-- struct SessionItem -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`SessionItemExt`][trait@crate::prelude::SessionItemExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait SessionItemExt -->
Trait containing all [`struct@SessionItem`] methods.

# Implementors

[`SessionItem`][struct@crate::SessionItem], [`SiAcquisition`][struct@crate::SiAcquisition], [`SiAdapter`][struct@crate::SiAdapter], [`SiEndpoint`][struct@crate::SiEndpoint], [`SiLink`][struct@crate::SiLink], [`SiLinkable`][struct@crate::SiLinkable]
<!-- impl SessionItem::fn handle_proxy_destroyed -->
Helper callback for sub-classes that deffers and unexports the session item.


Only meant to be used when the pipewire proxy destroyed signal is triggered.
## `proxy`
the proxy that was destroyed by the server
## `item`
the associated session item
<!-- impl SessionItem::fn make -->
Finds the factory associated with the given `name` from the `core` and uses it to construct a new WpSessionItem.
## `core`
the WpCore
## `factory_name`
the name of the factory to be used for constructing the object

# Returns

the new session item
<!-- trait SessionItemExt::fn configure -->
Configures the session item with a set of properties.
## `props`
the properties used to configure the item

# Returns

TRUE on success, FALSE if the item could not be configured
<!-- trait SessionItemExt::fn associated_proxy -->
An associated proxy is a WpProxy subclass instance that is somehow related to this item. For example:



 - An exported WpSiEndpoint should have at least:
 - an associated WpSiEndpoint
 - an associated WpSession

 - In cases where the item wraps a single PipeWire node, it should also have an associated WpNode
## `proxy_type`
a WpProxy subclass GType

# Returns

the associated proxy of the specified `proxy_type`, or NULL if there is no association to such a proxy
<!-- trait SessionItemExt::fn associated_proxy_id -->
Gets the bound id of a proxy associated with the session item.
## `proxy_type`
a WpProxy subclass GType

# Returns

the bound id of the associated proxy of the specified `proxy_type`, or SPA_ID_INVALID if there is no association to such a proxy
<!-- trait SessionItemExt::fn id -->
Gets the unique Id of the session item.
<!-- trait SessionItemExt::fn properties -->
Gets the properties of a session item.

# Returns

the item's properties.
<!-- trait SessionItemExt::fn property -->
Looks up a named session item property value for a given key.
## `key`
the property key

# Returns

the item property value for the given key.
<!-- trait SessionItemExt::fn is_configured -->
Checks if the session item is configured.

# Returns

TRUE if the item is configured, FALSE otherwise
<!-- trait SessionItemExt::fn register -->
Registers the session item to its associated core.
<!-- trait SessionItemExt::fn remove -->
Removes the session item from the registry.
<!-- trait SessionItemExt::fn reset -->
Resets the session item.


This essentially removes the configuration and deactivates all active features.
<!-- trait SessionItemExt::fn set_properties -->
Sets the item's properties.


This should only be done by sub-classes after the configuration has been done.
## `props`
the new properties to set
<!-- struct SessionItemFeatures -->
Flags to be used as WpObjectFeatures for WpSessionItem subclasses.
<!-- struct SiAcquisition -->


# Implements

[`SiAcquisitionExt`][trait@crate::prelude::SiAcquisitionExt], [`SessionItemExt`][trait@crate::prelude::SessionItemExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait SiAcquisitionExt -->
Trait containing all [`struct@SiAcquisition`] methods.

# Implementors

[`SiAcquisition`][struct@crate::SiAcquisition]
<!-- trait SiAcquisitionExtManual::fn acquire -->
Acquires the `item` for linking by `acquisitor`.


When a link is not allowed by policy, this operation should return an error.
When a link needs to be delayed for a short amount of time (ex. to apply a fade out effect on another item), this operation should finish with a delay. It is safe to assume that after this operation completes, the item will be linked immediately.
## `acquisitor`
the link that is trying to acquire a port info item
## `item`
the item that is being acquired
## `callback`
the callback to call when the operation is done
<!-- trait SiAcquisitionExt::fn release -->
Releases the `item`, which means that it is being unlinked.
## `acquisitor`
the link that had previously acquired the item
## `item`
the port info that is being released
<!-- struct SiAdapter -->


# Implements

[`SiAdapterExt`][trait@crate::prelude::SiAdapterExt], [`SessionItemExt`][trait@crate::prelude::SessionItemExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait SiAdapterExt -->
Trait containing all [`struct@SiAdapter`] methods.

# Implementors

[`SiAdapter`][struct@crate::SiAdapter]
<!-- trait SiAdapterExt::fn ports_format -->
Gets the format used to configure the adapter session item's ports.

# Returns

The format used to configure the ports of the adapter session item. Some items automatically choose a format when being activated, others never set a format on activation and the user needs to manually set it externally with [`SiAdapterExtManual::set_ports_format()`][crate::prelude::SiAdapterExtManual::set_ports_format()].

## `mode`
the mode
<!-- trait SiAdapterExtManual::fn set_ports_format -->
Sets the format and configures the adapter session item ports using the given format.


The result of the operation can be checked using the `wp_si_adapter_set_ports_format_finish()` API. If format is NULL, the adapter will be configured with the default format. If mode is NULL, the adapter will use "dsp" mode.
## `format`
the format to be set
## `mode`
the mode
## `callback`
the callback to call when the operation is done
<!-- struct SiEndpoint -->


# Implements

[`SiEndpointExt`][trait@crate::prelude::SiEndpointExt], [`SessionItemExt`][trait@crate::prelude::SessionItemExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait SiEndpointExt -->
Trait containing all [`struct@SiEndpoint`] methods.

# Implementors

[`SiEndpoint`][struct@crate::SiEndpoint]
<!-- trait SiEndpointExt::fn registration_info -->
This should return information that is used for registering the endpoint.


The return value should be a GVariant tuple of type (ssya{ss}) that contains, in order:
 - s: the endpoint's name
 - s: the media class
 - y: the direction
 - a{ss}: additional properties to be added to the list of global properties

# Returns

registration info for the endpoint
<!-- struct SiFactory -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`SiFactoryExt`][trait@crate::prelude::SiFactoryExt], [`trait@glib::ObjectExt`]
<!-- trait SiFactoryExt -->
Trait containing all [`struct@SiFactory`] methods.

# Implementors

[`SiFactory`][struct@crate::SiFactory]
<!-- impl SiFactory::fn new_simple -->
Creates a simple factory that constructs objects of a given GType.
## `factory_name`
the factory name; must be a static string!
## `si_type`
the WpSessionItem subclass type to instantiate for constructing items

# Returns

the new factory
<!-- impl SiFactory::fn find -->
Looks up a factory matching a name.
## `core`
the core
## `factory_name`
the lookup name

# Returns

the factory matching the lookup name
<!-- impl SiFactory::fn register -->
Registers the `factory` on the `core`.
## `core`
the core
## `factory`
the factory to register
<!-- trait SiFactoryExt::fn construct -->
Creates a new instance of the session item that is constructed by this factory.
## `core`
the core

# Returns

a new session item instance
<!-- trait SiFactoryExt::fn name -->
Gets the name of the factory.

# Returns

the factory name
<!-- struct SiLink -->


# Implements

[`SiLinkExt`][trait@crate::prelude::SiLinkExt], [`SessionItemExt`][trait@crate::prelude::SessionItemExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait SiLinkExt -->
Trait containing all [`struct@SiLink`] methods.

# Implementors

[`SiLink`][struct@crate::SiLink]
<!-- trait SiLinkExt::fn in_item -->
Gets the input item linked by the link.

# Returns

the input item that is linked by this link
<!-- trait SiLinkExt::fn out_item -->
Gets the output item linked by the link.

# Returns

the output item that is linked by this link
<!-- trait SiLinkExt::fn registration_info -->
This should return information that is used for registering the link, as a GVariant of type a{ss} that contains additional properties to be added to the list of global properties.

# Returns

registration info for the link
<!-- struct SiLinkable -->


# Implements

[`SiLinkableExt`][trait@crate::prelude::SiLinkableExt], [`SessionItemExt`][trait@crate::prelude::SessionItemExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- trait SiLinkableExt -->
Trait containing all [`struct@SiLinkable`] methods.

# Implementors

[`SiLinkable`][struct@crate::SiLinkable]
<!-- trait SiLinkableExt::fn acquisition -->
Gets the acquisition interface associated with the item.

# Returns

the acquisition interface associated with this item, or NULL if this item does not require acquiring items before linking them
<!-- trait SiLinkableExt::fn ports -->
This method returns a variant of type "a(uuu)", where each tuple in the array contains the following information:



 - u: (guint32) node id
 - u: (guint32) port id (the port must belong on the node specified above)
 - u: (guint32) the audio channel (enum spa_audio_channel) that this port makes available, or 0 for non-audio content

The order in which ports appear in this array is important when no channel information is available. The link implementation should link the ports in the order they appear. This is normally a good enough substitute for channel matching.
The `context` argument can be used to get different sets of ports from the item. The following well-known contexts are defined:
 - NULL: get the standard ports to be linked
 - "monitor": get the monitor ports
 - "control": get the control port
 - "reverse": get the reverse direction ports, if this item controls a filter node, which would have ports on both directions

Contexts other than NULL may only be used internally to ease the implementation of more complex item relationships. For example, a WpSessionItem that is in control of an input (sink) adapter node may implement WpSiLinkable where the NULL context will return the standard input ports and the "monitor" context will return the adapter's monitor ports. When linking this item to another item, the NULL context will always be used, but the item may internally spawn a secondary WpSessionItem that implements the "monitor" item. That secondary item may implement WpSiLinkable, chaining calls to the WpSiLinkable of the original item using the "monitor" context. This way, the monitor WpSessionItem does not need to share control of the underlying node; it only proxies calls to satisfy the API.
## `context`
an optional context for the ports

# Returns

a GVariant containing information about the ports of this item
<!-- struct SpaDevice -->


# Implements

[`ProxyExt`][trait@crate::prelude::ProxyExt], [`ObjectExt`][trait@crate::prelude::ObjectExt], [`trait@glib::ObjectExt`]
<!-- impl SpaDevice::fn from_spa_factory -->
Constructs a SPA_TYPE_INTERFACE_Device by loading the given SPA `factory_name`.


To export this device to the PipeWire server, you need to call [`ObjectExt::activate()`][crate::prelude::ObjectExt::activate()] requesting WP_PROXY_FEATURE_BOUND and wait for the operation to complete.
## `core`
the wireplumber core
## `factory_name`
the name of the SPA factory
## `properties`
properties to be passed to device constructor

# Returns

A new WpSpaDevice wrapping the device that was constructed by the factory, or NULL if the factory does not exist or was unable to construct the device
<!-- impl SpaDevice::fn new_wrap -->
Constructs an SPA Device object from an existing device handle.
## `core`
the wireplumber core
## `spa_device_handle`
the spa device handle
## `properties`
additional properties of the device

# Returns

A new WpSpaDevice
<!-- impl SpaDevice::fn managed_object -->
Gets one of the objects managed by this device.
## `id`
the (device-internal) id of the object to get

# Returns

the managed object associated with `id`
<!-- impl SpaDevice::fn properties -->
Gets the properties of this device.

# Returns

the device properties
<!-- impl SpaDevice::fn store_managed_object -->
Stores or removes a managed object into/from a device.
## `id`
the (device-internal) id of the object
## `object`
the object to store or NULL to remove the managed object associated with `id`
<!-- struct SpaDeviceFeatures -->
Flags to be used as WpObjectFeatures for WpSpaDevice.
<!-- struct SpaPod -->
<!-- impl SpaPod::fn new_boolean -->
Creates a spa pod of type boolean.
## `value`
the boolean value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_bytes -->
Creates a spa pod of type bytes.
## `value`
the bytes value
## `len`
the length of the bytes value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_choice -->
Creates a spa pod of type choice.
## `choice_type`
the name of the choice type ("Range", "Step", ...),

# Returns

The new spa pod
<!-- impl SpaPod::fn new_choice_valist -->
This is the va_list version of `wp_spa_pod_new_choice()`
## `choice_type`
the name of the choice type ("Range", "Step", ...)
## `args`
the variable arguments passed to `wp_spa_pod_new_choice()`

# Returns

The new spa pod
<!-- impl SpaPod::fn new_double -->
Creates a spa pod of type double.
## `value`
the double value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_fd -->
Creates a spa pod of type Fd.
## `value`
the Fd value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_float -->
Creates a spa pod of type float.
## `value`
the float value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_fraction -->
Creates a spa pod of type fraction.
## `num`
the numerator value of the fraction
## `denom`
the denominator value of the fraction

# Returns

The new spa pod
<!-- impl SpaPod::fn new_id -->
Creates a spa pod of type Id.
## `value`
the Id value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_int -->
Creates a spa pod of type int.
## `value`
the int value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_long -->
Creates a spa pod of type long.
## `value`
the long value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_none -->
Creates a spa pod of type None.

# Returns

The new spa pod
<!-- impl SpaPod::fn new_object -->
Creates a spa pod of type object.
## `type_name`
the type name of the object type
## `id_name`
the id name of the object,

# Returns

The new spa pod
<!-- impl SpaPod::fn new_object_valist -->
This is the va_list version of `wp_spa_pod_new_object()`
## `type_name`
the type name of the object type
## `id_name`
the id name of the object
## `args`
the variable arguments passed to `wp_spa_pod_new_object()`

# Returns

The new spa pod
<!-- impl SpaPod::fn new_pointer -->
Creates a spa pod of type pointer.
## `type_name`
the name of the type of the pointer
## `value`
the pointer value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_rectangle -->
Creates a spa pod of type rectangle.
## `width`
the width value of the rectangle
## `height`
the height value of the rectangle

# Returns

The new spa pod
<!-- impl SpaPod::fn new_sequence -->
Creates a spa pod of type sequence.
## `unit`
the unit of the sequence

# Returns

The new spa pod
<!-- impl SpaPod::fn new_sequence_valist -->
This is the va_list version of `wp_spa_pod_new_sequence()`
## `unit`
the unit of the sequence
## `args`
the variable arguments passed to `wp_spa_pod_new_sequence()`

# Returns

The new spa pod
<!-- impl SpaPod::fn new_string -->
Creates a spa pod of type string.
## `value`
the string value

# Returns

The new spa pod
<!-- impl SpaPod::fn new_wrap -->
Constructs a new WpSpaPod that wraps the given spa_pod.
## `pod`
a spa_pod

# Returns

a new WpSpaPod that references the data in `pod`. `pod` is not copied, so it needs to stay alive. The returned WpSpaPod can be modified by using the setter functions, in which case `pod` will be modified underneath.
<!-- impl SpaPod::fn new_wrap_const -->
Constructs a new immutable WpSpaPod that wraps the given spa_pod.
## `pod`
a constant spa_pod

# Returns

a new WpSpaPod that references the data in `pod`. `pod` is not copied, so it needs to stay alive. The returned WpSpaPod cannot be modified, unless it's copied first.
<!-- impl SpaPod::fn ensure_unique_owner -->
If `self` is not uniquely owned already, then it is unrefed and a copy of it is returned instead. You should always consider `self` as unsafe to use after this call and you should use the returned object instead.

# Returns

the uniquely owned spa pod object which may or may not be the same as `self`.
<!-- impl SpaPod::fn fixate -->
Fixates choices in an object pod so that they only have one value.

# Returns

TRUE if the pod was an object and it went through the fixation procedure, FALSE otherwise
<!-- impl SpaPod::fn array_child -->
Gets the child of a spa pod array object.

# Returns

the child of the spa pod array object
<!-- impl SpaPod::fn boolean -->
Gets the boolean value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the boolean value
<!-- impl SpaPod::fn bytes -->
Gets the bytes value and its len of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the bytes value

## `len`
the length of the bytes value
<!-- impl SpaPod::fn choice_child -->
Gets the child of a spa pod choice object.

# Returns

the child of the spa pod choice object
<!-- impl SpaPod::fn choice_type -->
If the pod is a Choice, this gets the choice type (Range, Step, Enum, ...)

# Returns

the choice type of the choice pod
<!-- impl SpaPod::fn control -->
Gets the offset, type name and spa pod value of a spa pod control.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `offset`
the offset of the control

## `ctl_type`
the control type (Properties, Midi, ...)

## `value`
the spa pod value of the control
<!-- impl SpaPod::fn double -->
Gets the double value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the double value
<!-- impl SpaPod::fn fd -->
Gets the Fd value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the Fd value
<!-- impl SpaPod::fn float -->
Gets the float value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the float value
<!-- impl SpaPod::fn fraction -->
Gets the fractions's numerator and denominator value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `num`
the fractions's numerator value

## `denom`
the fractions's denominator value
<!-- impl SpaPod::fn id -->
Gets the Id value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the Id value
<!-- impl SpaPod::fn int -->
Gets the int value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the int value
<!-- impl SpaPod::fn long -->
Gets the long value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the long value
<!-- impl SpaPod::fn object -->
Gets the object properties values of a spa pod object.

# Returns

TRUE if the object properties values were obtained, FALSE otherwise

## `id_name`
the id name of the object,
<!-- impl SpaPod::fn object_valist -->
This is the va_list version of `wp_spa_pod_get_object()`

# Returns

TRUE if the object properties values were obtained, FALSE otherwise

## `id_name`
the id name of the object

## `args`
the variable arguments passed to `wp_spa_pod_get_object()`
<!-- impl SpaPod::fn pointer -->
Gets the pointer value and its type name of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the pointer value
<!-- impl SpaPod::fn property -->
Gets the name, flags and spa pod value of a spa pod property.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `key`
the name of the property

## `value`
the spa pod value of the property
<!-- impl SpaPod::fn rectangle -->
Gets the rectangle's width and height value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `width`
the rectangle's width value

## `height`
the rectangle's height value
<!-- impl SpaPod::fn spa_pod -->
Converts a WpSpaPod pointer to a struct spa_pod one, for use with native pipewire & spa functions. The returned pointer is owned by WpSpaPod and may not be modified or freed.

# Returns

a const pointer to the underlying spa_pod structure
<!-- impl SpaPod::fn spa_type -->
Gets the SPA type of the spa pod.


If the pod is an object or pointer, this will return the derived object/pointer type directly. If the pod is an object property or a control, this will return the type of the contained value.

# Returns

the type of the spa pod
<!-- impl SpaPod::fn string -->
Gets the string value of a spa pod object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the string value
<!-- impl SpaPod::fn is_struct -->
Gets the struct's values of a spa pod object.

# Returns

TRUE if the struct values were obtained, FALSE otherwise
<!-- impl SpaPod::fn struct_valist -->
This is the va_list version of `wp_spa_pod_get_struct()`

# Returns

TRUE if the struct values were obtained, FALSE otherwise

## `args`
the variable arguments passed to `wp_spa_pod_get_struct()`
<!-- impl SpaPod::fn is_array -->
Checks wether the spa pod is of type array or not.

# Returns

TRUE if it is of type array, FALSE otherwise
<!-- impl SpaPod::fn is_boolean -->
Checks wether the spa pod is of type boolean or not.

# Returns

TRUE if it is of type boolean, FALSE otherwise
<!-- impl SpaPod::fn is_bytes -->
Checks wether the spa pod is of type bytes or not.

# Returns

TRUE if it is of type bytes, FALSE otherwise
<!-- impl SpaPod::fn is_choice -->
Checks wether the spa pod is of type choice or not.

# Returns

TRUE if it is of type choice, FALSE otherwise
<!-- impl SpaPod::fn is_control -->
Checks wether the spa pod is of type control or not.

# Returns

TRUE if it is of type control, FALSE otherwise
<!-- impl SpaPod::fn is_double -->
Checks wether the spa pod is of type double or not.

# Returns

TRUE if it is of type double, FALSE otherwise
<!-- impl SpaPod::fn is_fd -->
Checks wether the spa pod is of type Fd or not.

# Returns

TRUE if it is of type Fd, FALSE otherwise
<!-- impl SpaPod::fn is_float -->
Checks wether the spa pod is of type float or not.

# Returns

TRUE if it is of type float, FALSE otherwise
<!-- impl SpaPod::fn is_fraction -->
Checks wether the spa pod is of type fraction or not.

# Returns

TRUE if it is of type fraction, FALSE otherwise
<!-- impl SpaPod::fn is_id -->
Checks wether the spa pod is of type Id or not.

# Returns

TRUE if it is of type Id, FALSE otherwise
<!-- impl SpaPod::fn is_int -->
Checks wether the spa pod is of type int or not.

# Returns

TRUE if it is of type int, FALSE otherwise
<!-- impl SpaPod::fn is_long -->
Checks wether the spa pod is of type long or not.

# Returns

TRUE if it is of type long, FALSE otherwise
<!-- impl SpaPod::fn is_none -->
Checks wether the spa pod is of type none or not.

# Returns

TRUE if it is of type none, FALSE otherwise
<!-- impl SpaPod::fn is_object -->
Checks wether the spa pod is of type object or not.

# Returns

TRUE if it is of type object, FALSE otherwise
<!-- impl SpaPod::fn is_pointer -->
Checks wether the spa pod is of type pointer or not.

# Returns

TRUE if it is of type pointer, FALSE otherwise
<!-- impl SpaPod::fn is_property -->
Checks wether the spa pod is of type property or not.

# Returns

TRUE if it is of type property, FALSE otherwise
<!-- impl SpaPod::fn is_rectangle -->
Checks wether the spa pod is of type rectangle or not.

# Returns

TRUE if it is of type rectangle, FALSE otherwise
<!-- impl SpaPod::fn is_sequence -->
Checks wether the spa pod is of type sequence or not.

# Returns

TRUE if it is of type sequence, FALSE otherwise
<!-- impl SpaPod::fn is_string -->
Checks wether the spa pod is of type string or not.

# Returns

TRUE if it is of type string, FALSE otherwise
<!-- impl SpaPod::fn is_struct -->
Checks wether the spa pod is of type struct or not.

# Returns

TRUE if it is of type struct, FALSE otherwise
<!-- impl SpaPod::fn is_unique_owner -->
Checks if the pod is the unique owner of its data or not.

# Returns

TRUE if the pod owns the data, FALSE otherwise.
<!-- impl SpaPod::fn new_iterator -->
Creates a new iterator for a spa pod object.

# Returns

the new spa pod iterator
<!-- impl SpaPod::fn set_boolean -->
Sets a boolean value in the spa pod object.
## `value`
the boolean value

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_double -->
Sets a double value in the spa pod object.
## `value`
the double value

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_fd -->
Sets a Fd value in the spa pod object.
## `value`
the Fd value

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_float -->
Sets a float value in the spa pod object.
## `value`
the float value

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_fraction -->
Sets the numerator and denominator values of a fraction in the spa pod object.
## `num`
the numerator value of the farction
## `denom`
the denominator value of the fraction

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_id -->
Sets an Id value in the spa pod object.
## `value`
the Id value

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_int -->
Sets an int value in the spa pod object.
## `value`
the int value

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_long -->
Sets a long value in the spa pod object.
## `value`
the long value

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_pod -->
Sets the value of a spa pod object in the current spa pod object. The spa pod objects must be of the same value.
## `pod`
the pod with the value to be set

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_pointer -->
Sets a pointer value with its type name in the spa pod object.
## `type_name`
the name of the type of the pointer
## `value`
the pointer value

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- impl SpaPod::fn set_rectangle -->
Sets the width and height values of a rectangle in the spa pod object.
## `width`
the width value of the rectangle
## `height`
the height value of the rectangle

# Returns

TRUE if the value could be set, FALSE othewrise.
<!-- struct SpaPodBuilder -->
<!-- impl SpaPodBuilder::fn new_array -->
Creates a spa pod builder of type array.

# Returns

the new spa pod builder
<!-- impl SpaPodBuilder::fn new_choice -->
Creates a spa pod builder of type choice.
## `choice_type`
the name of the choice type ("Range", "Step", ...)

# Returns

the new spa pod builder
<!-- impl SpaPodBuilder::fn new_object -->
Creates a spa pod builder of type object.
## `type_name`
the type name of the object type
## `id_name`
the Id name of the object

# Returns

the new spa pod builder
<!-- impl SpaPodBuilder::fn new_sequence -->
Creates a spa pod builder of type sequence.

# Returns

the new spa pod builder
<!-- impl SpaPodBuilder::fn new_struct -->
Creates a spa pod builder of type struct.

# Returns

the new spa pod builder
<!-- impl SpaPodBuilder::fn add -->
Adds a list of values into the builder.
<!-- impl SpaPodBuilder::fn add_boolean -->
Adds a boolean value into the builder.
## `value`
the boolean value
<!-- impl SpaPodBuilder::fn add_bytes -->
Adds a bytes value with its length into the builder.
## `value`
the bytes value
## `len`
the length of the bytes value
<!-- impl SpaPodBuilder::fn add_control -->
Adds a control into the builder.
## `offset`
the offset of the control
## `ctl_type`
the type name of the control
<!-- impl SpaPodBuilder::fn add_double -->
Adds a double value into the builder.
## `value`
the double value
<!-- impl SpaPodBuilder::fn add_fd -->
Adds a Fd value into the builder.
## `value`
the Fd value
<!-- impl SpaPodBuilder::fn add_float -->
Adds a float value into the builder.
## `value`
the float value
<!-- impl SpaPodBuilder::fn add_fraction -->
Adds the numerator and denominator values of a fraction into the builder.
## `num`
the numerator value of the fraction
## `denom`
the denominator value of the fraction
<!-- impl SpaPodBuilder::fn add_id -->
Adds a Id value into the builder.
## `value`
the Id value
<!-- impl SpaPodBuilder::fn add_int -->
Adds a int value into the builder.
## `value`
the int value
<!-- impl SpaPodBuilder::fn add_long -->
Adds a long value into the builder.
## `value`
the long value
<!-- impl SpaPodBuilder::fn add_none -->
Adds a none value into the builder.
<!-- impl SpaPodBuilder::fn add_pod -->
Adds a pod value into the builder.
## `pod`
the pod value
<!-- impl SpaPodBuilder::fn add_pointer -->
Adds a pointer value with its type name into the builder.
## `type_name`
the type name that the pointer points to
## `value`
the pointer vaue
<!-- impl SpaPodBuilder::fn add_property -->
Adds a property into the builder.
## `key`
the name of the property
<!-- impl SpaPodBuilder::fn add_property_id -->
Adds a property into the builder.
## `id`
the id of the property
<!-- impl SpaPodBuilder::fn add_rectangle -->
Adds the width and height values of a rectangle into the builder.
## `width`
the width value of the rectangle
## `height`
the height value of the rectangle
<!-- impl SpaPodBuilder::fn add_string -->
Adds a string value into the builder.
## `value`
the string value
<!-- impl SpaPodBuilder::fn add_valist -->
Adds a list of values into the builder.
## `args`
the variable arguments passed to `wp_spa_pod_builder_add()`
<!-- impl SpaPodBuilder::fn end -->
Ends the builder process and returns the constructed spa pod object.

# Returns

the constructed spa pod object
<!-- struct SpaPodParser -->
<!-- impl SpaPodParser::fn new_struct -->
Creates an struct spa pod parser. The `pod` object must be valid for the entire life-cycle of the returned parser.
## `pod`
the struct spa pod to parse

# Returns

The new spa pod parser
<!-- impl SpaPodParser::fn end -->
Ends the parser process.
<!-- impl SpaPodParser::fn get -->
Gets a list of values from a spa pod parser object.

# Returns

TRUE if the values were obtained, FALSE otherwise
<!-- impl SpaPodParser::fn boolean -->
Gets the boolean value from a spa pod parser.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the boolean value
<!-- impl SpaPodParser::fn bytes -->
Gets the bytes value and its length from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the bytes value

## `len`
the length of the bytes value
<!-- impl SpaPodParser::fn double -->
Gets the double value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the double value
<!-- impl SpaPodParser::fn fd -->
Gets the Fd value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the Fd value
<!-- impl SpaPodParser::fn float -->
Gets the float value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the float value
<!-- impl SpaPodParser::fn fraction -->
Gets the fractions's numerator and denominator value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `num`
the fractions's numerator value

## `denom`
the fractions's denominator value
<!-- impl SpaPodParser::fn id -->
Gets the Id value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the Id value
<!-- impl SpaPodParser::fn int -->
Gets the int value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the int value
<!-- impl SpaPodParser::fn long -->
Gets the long value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the long value
<!-- impl SpaPodParser::fn pod -->
Gets the spa pod value from a spa pod parser object.

# Returns

The spa pod value or NULL if it could not be obtained
<!-- impl SpaPodParser::fn pointer -->
Gets the pointer value and its type name from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the pointer value
<!-- impl SpaPodParser::fn rectangle -->
Gets the rectangle's width and height value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `width`
the rectangle's width value

## `height`
the rectangle's height value
<!-- impl SpaPodParser::fn string -->
Gets the string value from a spa pod parser object.

# Returns

TRUE if the value was obtained, FALSE otherwise

## `value`
the string value
<!-- impl SpaPodParser::fn is_valist -->
This is the va_list version of `wp_spa_pod_parser_get()`
## `args`
the variable arguments passed to `wp_spa_pod_parser_get()`

# Returns

TRUE if the values were obtained, FALSE otherwise
<!-- struct SpaType -->

<!-- impl SpaType::fn from_name -->
Looks up the type id from a given type name.
## `name`
the name to look up

# Returns

the corresponding type id or WP_SPA_TYPE_INVALID if not found
<!-- impl SpaType::fn object_id_values_table -->
Gets the table with the values that can be stored in the special "id" field of an object of the given `self`.


Object pods (see WpSpaPod) always have a special "id" field along with other fields that can be defined. This "id" field can only store values of a specific SPA_TYPE_Id type. This function returns the table that contains the possible values for that field.

# Returns

the table with the values that can be stored in the special "id" field of an object of the given `self`
<!-- impl SpaType::fn values_table -->
Gets the values table of an SPA type.

# Returns

the associated WpSpaIdTable that contains possible values or object fields for this type, or NULL
<!-- impl SpaType::fn is_fundamental -->
Checks if an SPA type is a fundamental type.

# Returns

TRUE if the `self` has no parent, FALSE otherwise
<!-- impl SpaType::fn is_id -->
Checks if an SPA type is an Id type.

# Returns

TRUE if the `self` is a SPA_TYPE_Id, FALSE otherwise
<!-- impl SpaType::fn is_object -->
Checks if an SPA type is an Object type.

# Returns

TRUE if the `self` is a SPA_TYPE_Object, FALSE otherwise
<!-- impl SpaType::fn name -->
Gets the name of an SPA type.

# Returns

the complete name of the given `self` or NULL if `self` is invalid
<!-- impl SpaType::fn parent -->
Gets the parent type of an SPA type.

# Returns

the direct parent type of the given `self`; if the type is fundamental (i.e. has no parent), the returned type is the same as `self`
<!-- struct State -->


# Implements

[`trait@glib::ObjectExt`]
<!-- impl State::fn new -->
Constructs a new state object.
## `name`
the state name

# Returns

the new WpState
<!-- impl State::fn clear -->
Clears the state removing its file.
<!-- impl State::fn location -->
Gets the location of a state object.

# Returns

the location of this state
<!-- impl State::fn name -->
Gets the name of a state object.

# Returns

the name of this state
<!-- impl State::fn load -->
Loads the state data from the file system.


This function will never fail. If it cannot load the state, for any reason, it will simply return an empty WpProperties, behaving as if there was no previous state stored.

# Returns

a new WpProperties containing the state data
<!-- impl State::fn save -->
Saves new properties in the state, overwriting all previous data.
## `props`
the properties to save

# Returns

TRUE if the properties could be saved, FALSE otherwise
<!-- struct Transition -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`TransitionExt`][trait@crate::prelude::TransitionExt], [`trait@glib::ObjectExt`], [`trait@gio::prelude::AsyncResultExt`]
<!-- trait TransitionExt -->
Trait containing all [`struct@Transition`] methods.

# Implementors

[`FeatureActivationTransition`][struct@crate::FeatureActivationTransition], [`Transition`][struct@crate::Transition]
<!-- impl Transition::fn new -->
Creates a WpTransition acting on `source_object`.


When the transition is done, `callback` will be invoked.
The transition does not automatically start executing steps. You must call [`TransitionExt::advance()`][crate::prelude::TransitionExt::advance()] after creating it in order to start it.
The transition is automatically unref'ed after the `callback` has been executed. If you wish to keep an additional reference on it, you need to ref it explicitly.
## `type_`
the GType of the WpTransition subclass to instantiate
## `source_object`
the GObject that owns this task, or NULL
## `cancellable`
optional GCancellable
## `callback`
a GAsyncReadyCallback
## `callback_data`
user data passed to `callback`

# Returns

the new transition
<!-- impl Transition::fn new_closure -->
Creates a WpTransition acting on `source_object`. When the transition is done, `closure` will be invoked.


The transition does not automatically start executing steps. You must call [`TransitionExt::advance()`][crate::prelude::TransitionExt::advance()] after creating it in order to start it.
Note that the transition is automatically unref'ed after the `closure` has been executed. If you wish to keep an additional reference on it, you need to ref it explicitly.
## `type_`
the GType of the WpTransition subclass to instantiate
## `source_object`
the GObject that owns this task, or NULL
## `cancellable`
optional GCancellable
## `closure`
a GAsyncReadyCallback wrapped in a GClosure

# Returns

the new transition
<!-- impl Transition::fn finish -->
Returns the final return status of the transition and its error, if there was one.


This is meant to be called from within the GAsyncReadyCallback that was specified in `wp_transition_new()`.
## `res`
a transition, as a GAsyncResult

# Returns

TRUE if the transition completed successfully, FALSE if there was an error
<!-- trait TransitionExt::fn advance -->
Advances the transition to the next step.


This initially calls `_WpTransitionClass::get_next_step()` in order to determine what the next step is. If `_WpTransitionClass::get_next_step()` returns a step different than the previous one, it calls `_WpTransitionClass::execute_step()` to execute it.
The very first time that `_WpTransitionClass::get_next_step()` is called, its `step` parameter equals WP_TRANSITION_STEP_NONE.
When `_WpTransitionClass::get_next_step()` returns WP_TRANSITION_STEP_NONE this function completes the transition, calling the transition's callback and then unref-ing the transition.
When `_WpTransitionClass::get_next_step()` returns WP_TRANSITION_STEP_ERROR, this function calls [`TransitionExtManual::return_error()`][crate::prelude::TransitionExtManual::return_error()], unless it has already been called directly by `_WpTransitionClass::get_next_step()`.
In error conditions, `_WpTransitionClass::execute_step()` is called once with `step` being WP_TRANSITION_STEP_ERROR, allowing the implementation to rollback any changes or cancel underlying jobs, if necessary.
<!-- trait TransitionExt::fn is_completed -->
Checks if the transition completed.

# Returns

TRUE if the transition has completed (with or without an error), FALSE otherwise
<!-- trait TransitionExt::fn data -->
Gets `self` 's data.


See `wp_transition_set_data()`.

# Returns

the transition's data
<!-- trait TransitionExt::fn source_object -->
Gets the source object from the transition.


Like [`AsyncResultExtManual::source_object()`][crate::gio::prelude::AsyncResultExtManual::source_object()], but does not ref the object.

# Returns

the source object
<!-- trait TransitionExt::fn source_tag -->
Gets `self` 's source tag.


See `wp_transition_set_source_tag()`.

# Returns

the transition's source tag
<!-- trait TransitionExt::fn had_error -->
Checks if the transition completed with an error.

# Returns

TRUE if the transition completed with an error, FALSE otherwise
<!-- trait TransitionExt::fn is_tagged -->
Checks if `self` has the given `tag` (generally a function pointer indicating the function `self` was created by).
## `tag`
a tag

# Returns

TRUE if `self` has the indicated `tag` , FALSE if not.
<!-- trait TransitionExtManual::fn return_error -->
Completes the transition with an error.


This can be called anytime from within any virtual function or an async job handler.
In most cases this will also unref the transition, so it is not safe to access it after this function has been called.
## `error`
a GError
<!-- trait TransitionExt::fn set_data -->
Sets `self` 's data (freeing the existing data, if any). This can be an arbitrary user structure that holds data associated with this transition.
## `data_destroy`
GDestroyNotify for `data`
<!-- trait TransitionExt::fn set_source_tag -->
Sets `self` 's source tag.


You can use this to tag a transition's return value with a particular pointer (usually a pointer to the function doing the tagging) and then later check it using `wp_transition_get_source_tag()` (or [`AsyncResultExtManual::is_tagged()`][crate::gio::prelude::AsyncResultExtManual::is_tagged()]) in the transition's "finish" function, to figure out if the response came from a particular place.
## `tag`
an opaque pointer indicating the source of this transition
<!-- enum TransitionStep -->
Values for the `steps` of the implemented state machine.
