"use strict";
/*
 * Copyright 2016 Palantir Technologies, Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
Object.defineProperty(exports, "__esModule", { value: true });
var tslib_1 = require("tslib");
var classnames_1 = tslib_1.__importDefault(require("classnames"));
var React = tslib_1.__importStar(require("react"));
var react_lifecycles_compat_1 = require("react-lifecycles-compat");
var common_1 = require("../../common");
var props_1 = require("../../common/props");
var icon_1 = require("../icon/icon");
var DEFAULT_RIGHT_ELEMENT_WIDTH = 10;
var InputGroup = /** @class */ (function (_super) {
    tslib_1.__extends(InputGroup, _super);
    function InputGroup() {
        var _this = _super !== null && _super.apply(this, arguments) || this;
        _this.state = {
            rightElementWidth: DEFAULT_RIGHT_ELEMENT_WIDTH,
        };
        _this.refHandlers = {
            rightElement: function (ref) { return (_this.rightElement = ref); },
        };
        return _this;
    }
    InputGroup.prototype.render = function () {
        var _a;
        var _b = this.props, className = _b.className, disabled = _b.disabled, fill = _b.fill, intent = _b.intent, large = _b.large, small = _b.small, leftIcon = _b.leftIcon, round = _b.round;
        var classes = classnames_1.default(common_1.Classes.INPUT_GROUP, common_1.Classes.intentClass(intent), (_a = {},
            _a[common_1.Classes.DISABLED] = disabled,
            _a[common_1.Classes.FILL] = fill,
            _a[common_1.Classes.LARGE] = large,
            _a[common_1.Classes.SMALL] = small,
            _a[common_1.Classes.ROUND] = round,
            _a), className);
        var style = tslib_1.__assign({}, this.props.style, { paddingRight: this.state.rightElementWidth });
        return (React.createElement("div", { className: classes },
            React.createElement(icon_1.Icon, { icon: leftIcon }),
            React.createElement("input", tslib_1.__assign({ type: "text" }, props_1.removeNonHTMLProps(this.props), { className: common_1.Classes.INPUT, ref: this.props.inputRef, style: style })),
            this.maybeRenderRightElement()));
    };
    InputGroup.prototype.componentDidMount = function () {
        if (this.rightElement != null) {
            var clientWidth = this.rightElement.clientWidth;
            // small threshold to prevent infinite loops
            if (Math.abs(clientWidth - this.state.rightElementWidth) > 2) {
                this.setState({ rightElementWidth: clientWidth });
            }
        }
    };
    InputGroup.prototype.maybeRenderRightElement = function () {
        var rightElement = this.props.rightElement;
        if (rightElement == null) {
            return undefined;
        }
        return (React.createElement("span", { className: common_1.Classes.INPUT_ACTION, ref: this.refHandlers.rightElement }, rightElement));
    };
    InputGroup.displayName = props_1.DISPLAYNAME_PREFIX + ".InputGroup";
    InputGroup = tslib_1.__decorate([
        react_lifecycles_compat_1.polyfill
    ], InputGroup);
    return InputGroup;
}(common_1.AbstractPureComponent2));
exports.InputGroup = InputGroup;
//# sourceMappingURL=inputGroup.js.map