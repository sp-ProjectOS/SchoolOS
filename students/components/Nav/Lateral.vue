<template>
    <div class="lateral">
        <div class="lateral-close" @click="close">
            <Icon name="close" :size="4" color="var(--bg-secondary)" />
        </div>
        <div class="lateral-option">
            <Icon name="account_circle" :size="4" color="var(--bg-secondary)" />
            <p>My account</p>
        </div>
        <div class="qr"></div>
        <div class="lateral-option">
            <Icon name="calendar_month" :size="4" color="var(--bg-secondary)" />
            <p>My Schedule</p>
        </div>
        <div class="lateral-option">
            <Icon name="qr_code" :size="4" color="var(--bg-secondary)" />
            <p>My Schedule</p>
        </div>
    </div>
</template>
<style scoped>
.lateral {
    width: 70%;
    background-color: var(--bg-primary);
    position: absolute;
    top: 0;
    right: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    z-index: 10;
}

/* .lateral:first-child {
    justify-content: flex-start;
}
 */
.lateral-close {
    top: 0.5rem;
    right: 0.5rem;
    position: absolute;
    cursor: pointer;
}

.lateral-option {
    background-color: var(--color-secondary);
    display: flex;
    margin: 1rem 0;
    cursor: pointer;
}

.lateral-option p {
    color: white;
    font-size: 1.2rem;
}
</style>
<script lang="ts" setup>
// Emit an event to the parent component when the user clicks on the close button
const emit = defineEmits(['close']);

const close = () => {
	emit('close');
};
</script>