/*
 *   Copyright (c) 2022 R3BL LLC
 *   All rights reserved.
 *
 *   Licensed under the Apache License, Version 2.0 (the "License");
 *   you may not use this file except in compliance with the License.
 *   You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 *   Unless required by applicable law or agreed to in writing, software
 *   distributed under the License is distributed on an "AS IS" BASIS,
 *   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *   See the License for the specific language governing permissions and
 *   limitations under the License.
 */

use std::fmt::Debug;

use async_trait::async_trait;
use r3bl_rs_utils_core::*;

use crate::*;

/// See [App].
#[async_trait]
pub trait Component<S, A>
where
    S: Debug + Default + Clone + PartialEq + Sync + Send,
    A: Debug + Default + Clone + Sync + Send,
{
    /// This is an optional method that can be used to initialize the state of the component's
    /// engines. This applies to modal dialog components that need their engine to be initialized
    /// before they are shown / activated.
    fn reset(&mut self);

    fn get_id(&self) -> FlexBoxId;

    /// Use the state to render the output. The state is immutable. If you want to change it then it
    /// should be done in the [Component::handle_event] method. Here are all the arguments that are
    /// passed in (which can be used to render the output):
    ///
    /// - Arguments:
    ///   - Get from `current_box`:
    ///     - box_origin_pos: Position
    ///     - box_bounding_size: Size
    ///     - maybe_box_style: `Option<Style>`
    ///   - Get from `state`:
    ///     - Content to render
    ///     - get_focus_id(): String to determine if this component has keyboard focus (might affect
    ///       the way it gets rendered)
    ///   - Maybe use `shared_store`:
    ///     - Dispatch an action if needed
    ///   - Maybe use `surface`:
    ///     - Get the origin and size of the surface that can be drawn to (maybe different than the
    ///       size of the window)
    ///
    /// - Returns:
    ///   - [RenderPipeline] which must be rendered by the caller
    ///
    /// - Clipping, scrolling, overdrawing:
    ///   - Each implementation of this trait is solely responsible of taking care of these
    ///     behaviors
    async fn render(
        &mut self,
        args: ComponentScopeArgs<'_, S, A>,
        current_box: &FlexBox,
        surface_bounds: SurfaceBounds,
    ) -> CommonResult<RenderPipeline>;

    /// If this component has focus [HasFocus] then this method will be called to handle input event
    /// that is meant for it.
    ///
    /// More granularly, here is the journey:
    /// 1. This method might end up calling on an underlying engine function & pass the
    ///    `input_event` & state (from the redux store) to it.
    ///    - Engines tend to have a corresponding `apply_event` method which returns a new result or
    ///      response type, eg: [DialogEngineApplyResponse] or [EditorEngineApplyEventResult].
    /// 2. Then the response or result is used to run a callback function that was passed in when
    ///    the component was created (which will then end up dispatching an action to the redux
    ///    store).
    /// 3. Finally an [EventPropagation] is returned to let the caller know whether the
    ///    `input_event` was consumed or not & whether it should re-render (outside of a redux store
    ///    state change).
    async fn handle_event(
        &mut self,
        args: ComponentScopeArgs<'_, S, A>,
        input_event: &InputEvent,
    ) -> CommonResult<EventPropagation>;
}

#[async_trait]
pub trait SurfaceRender<S, A>
where
    S: Debug + Default + Clone + PartialEq + Sync + Send,
    A: Debug + Default + Clone + Sync + Send,
{
    async fn render_in_surface(
        &mut self,
        args: GlobalScopeArgs<'_, S, A>,
        surface: &mut Surface,
    ) -> CommonResult<()>;
}
